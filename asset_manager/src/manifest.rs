use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::{AssetError, AssetResult};

#[derive(Debug, Clone)]
pub struct Sourced<T> {
    pub value: T,
    pub source: std::path::PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ManifestRoot {
    pub version: u32,

    #[serde(default)]
    pub asset_definition_files: Vec<String>,

    #[serde(default)]
    pub asset_definitions: Option<AssetDefinitions>,
}

#[derive(Debug, Default, Clone)]
pub struct AssetDefinitionsSourced {
    pub sprites: Vec<Sourced<SpriteDefinition>>,
    pub spritesheets: Vec<Sourced<SpriteSheetDefinition>>,
    pub animations: Vec<Sourced<AnimationDefinition>>,
    pub audio: Vec<Sourced<AudioDefinition>>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct AssetDefinitions {
    #[serde(default)]
    pub sprites: Vec<SpriteDefinition>,

    #[serde(default)]
    pub spritesheets: Vec<SpriteSheetDefinition>,

    #[serde(default)]
    pub animations: Vec<AnimationDefinition>,

    #[serde(default)]
    pub audio: Vec<AudioDefinition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpriteDefinition {
    pub key: String,

    // Optional in TOML because sprites inside a spritesheet inherit it.
    #[serde(default)]
    pub file_path: Option<String>,

    pub size: [i32; 2],
    pub offset: [i32; 2],
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpriteSheetDefinition {
    pub key: String,
    pub file_path: String,
    pub quantity: i32,

    #[serde(default)]
    pub sprites: Vec<SpriteDefinition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnimationDefinition {
    pub key: String,
    pub framerate: i32,
    pub sprite_keys: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AudioDefinition {
    pub key: String,
    pub file_path: String,
    pub time_offset: i32, // ms
    pub duration: i32,    // ms
}

/// Final merged + validated output that the engine will use.
/// Paths are resolved to absolute/normalized `PathBuf`s relative to manifest.
/// Sprites from sheets are flattened into `sprites` too.
#[derive(Debug)]
pub struct LoadedDefinitions {
    pub sprites: Vec<ResolvedSprite>,
    pub animations: Vec<AnimationDefinition>,
    pub audio: Vec<ResolvedAudio>,
}

#[derive(Debug, Clone)]
pub enum SpriteSource {
    Standalone { file_path: PathBuf},
    Sheet { sheet_path: PathBuf, offset: [i32; 2], size: [i32; 2] },
}

#[derive(Debug, Clone)]
pub struct ResolvedSprite {
    pub key: String,
    pub source: PathBuf,
    pub sprite_source: SpriteSource,
    pub offset : [i32; 2],
    pub size : [i32; 2],
}

#[derive(Debug, Clone)]
pub struct ResolvedAudio {
    pub key: String,
    pub file_path: PathBuf,
    pub time_offset: i32,
    pub duration: i32,
    pub source: PathBuf,
}

pub fn load_manifest_and_definitions(manifest_path: impl AsRef<Path>,) -> AssetResult<LoadedDefinitions> {
    let manifest_path = manifest_path.as_ref().to_path_buf();
    let manifest_dir = manifest_path
        .parent()
        .unwrap_or(Path::new("."))
        .to_path_buf();

    let manifest_text = fs::read_to_string(&manifest_path).map_err(|e| AssetError::Io {
        path: manifest_path.clone(),
        source: e,
    })?;

    let root: ManifestRoot = toml::from_str(&manifest_text).map_err(|e| AssetError::TomlParse {
        path: manifest_path.clone(),
        source: e,
    })?;

    if root.version != 1 {
        return Err(AssetError::InvalidManifest(format!(
            "Unsupported manifest version: {}",
            root.version
        )));
    }

    // Merge sourced definitions from referenced files, then inline last.
    let mut merged = AssetDefinitionsSourced::default();

    for rel in &root.asset_definition_files {
        let def_path = manifest_dir.join(rel);
        let defs = load_asset_definitions_file(&def_path)?; // already sourced
        merged = merge_sourced(merged, defs);
    }

    if let Some(inline) = root.asset_definitions {
        let inline_sourced = attach_source(inline, manifest_path.clone());
        merged = merge_sourced(merged, inline_sourced);
    }

    resolve_and_validate(merged, manifest_dir)
}

fn load_asset_definitions_file(path: &Path) -> AssetResult<AssetDefinitionsSourced> {
    let text = fs::read_to_string(path).map_err(|e| AssetError::Io {
        path: path.to_path_buf(),
        source: e,
    })?;

    #[derive(Deserialize)]
    struct Wrapper {
        #[serde(default)]
        asset_definitions: AssetDefinitions,
    }

    let wrapper: Wrapper = toml::from_str(&text).map_err(|e| AssetError::TomlParse {
        path: path.to_path_buf(),
        source: e,
    })?;

    Ok(attach_source(wrapper.asset_definitions, path.to_path_buf()))
}

fn attach_source(defs: AssetDefinitions, source: PathBuf) -> AssetDefinitionsSourced {
    AssetDefinitionsSourced {
        sprites: defs
            .sprites
            .into_iter()
            .map(|v| Sourced { value: v, source: source.clone() })
            .collect(),
        spritesheets: defs
            .spritesheets
            .into_iter()
            .map(|v| Sourced { value: v, source: source.clone() })
            .collect(),
        animations: defs
            .animations
            .into_iter()
            .map(|v| Sourced { value: v, source: source.clone() })
            .collect(),
        audio: defs
            .audio
            .into_iter()
            .map(|v| Sourced { value: v, source: source.clone() })
            .collect(),
    }
}

fn merge_sourced(mut a: AssetDefinitionsSourced,
                     b: AssetDefinitionsSourced,
                ) -> AssetDefinitionsSourced {
    a.sprites.extend(b.sprites);
    a.spritesheets.extend(b.spritesheets);
    a.animations.extend(b.animations);
    a.audio.extend(b.audio);
    a
}

fn resolve_and_validate(defs: AssetDefinitionsSourced, manifest_dir: PathBuf,) -> AssetResult<LoadedDefinitions> {
    let mut sprite_key_source: HashMap<String, PathBuf> = HashMap::new();
    let mut animation_key_source: HashMap<String, PathBuf> = HashMap::new();
    let mut audio_key_source: HashMap<String, PathBuf> = HashMap::new();

    let mut resolved_sprites: Vec<ResolvedSprite> = Vec::new();

    // Standalone sprites
    for s in defs.sprites {
        let key = s.value.key.clone();

        let fp = s.value.file_path.clone().ok_or_else(|| AssetError::MissingSpriteFilePath {
            key: key.clone(),
            source: s.source.clone(),
        })?;

        ensure_unique_key("sprite", &key, &mut sprite_key_source, &s.source)?;

        resolved_sprites.push(ResolvedSprite {
            key,
            source: s.source.clone(),
            sprite_source: SpriteSource::Standalone {
                file_path: resolve_path(&manifest_dir, &fp),
            },
            size: s.value.size,
            offset:s.value.offset,
        });
    }

    // Spritesheets -> flatten sprites; nested sprite provenance = sheet source
    for sheet in defs.spritesheets {
        for s in sheet.value.sprites {
            let key = s.key.clone();

            ensure_unique_key("sprite", &key, &mut sprite_key_source, &sheet.source)?;

            let sheet_path = resolve_path(&manifest_dir, &sheet.value.file_path);

            resolved_sprites.push(ResolvedSprite {
                key,
                source: sheet.source.clone(),
                sprite_source: SpriteSource::Sheet {
                    sheet_path,
                    offset: s.offset,
                    size: s.size,
                },
                size: s.size,
                offset: s.offset,
            });
        }
    }

    // Audio
    let mut resolved_audio: Vec<ResolvedAudio> = Vec::new();
    for a in defs.audio {
        ensure_unique_key("audio", &a.value.key, &mut audio_key_source, &a.source)?;

        resolved_audio.push(ResolvedAudio {
            key: a.value.key,
            file_path: resolve_path(&manifest_dir, &a.value.file_path),
            time_offset: a.value.time_offset,
            duration: a.value.duration,
            source: a.source.clone(),
        });
    }

    // Validate animations
    let sprite_keys: HashSet<String> = resolved_sprites.iter().map(|s| s.key.clone()).collect();

    // move animations out, validate them, then return them
    let mut animations: Vec<AnimationDefinition> = Vec::new();
    for anim in defs.animations {
        ensure_unique_key(
            "animation",
            &anim.value.key,
            &mut animation_key_source,
            &anim.source,
        )?;

        for sk in &anim.value.sprite_keys {
            if !sprite_keys.contains(sk) {
                return Err(AssetError::AnimationReferencesMissingSprite {
                    animation_key: anim.value.key.clone(),
                    missing_sprite_key: sk.clone(),
                    source: anim.source.clone(),
                });
            }
        }

        animations.push(anim.value);
    }

    Ok(LoadedDefinitions {
        sprites: resolved_sprites,
        animations,
        audio: resolved_audio,
    })
}

fn ensure_unique_key(
    asset_type: &'static str,
    key: &str,
    seen: &mut HashMap<String, PathBuf>,
    source: &PathBuf,
) -> AssetResult<()> {
    if let Some(first_source) = seen.get(key) {
        return Err(AssetError::DuplicateKey {
            asset_type,
            key: key.to_string(),
            first_source: first_source.clone(),
            second_source: source.clone(),
        });
    }
    seen.insert(key.to_string(), source.clone());
    Ok(())
}

fn resolve_path(manifest_dir: &Path, raw: &str) -> PathBuf {
    let p = PathBuf::from(raw);
    if p.is_absolute() {
        p
    } else {
        manifest_dir.join(p)
    }
}