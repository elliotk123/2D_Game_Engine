use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::{AssetError, AssetResult};
use crate::manifest::{LoadedDefinitions, SpriteSource};

#[derive(Debug, Clone)]
pub struct SpriteAsset {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>, // RGBA8888
}

#[derive(Debug, Clone)]
enum SpriteRef {
    Standalone { path: PathBuf },
    SheetRegion { sheet_path: PathBuf, offset: [i32; 2], size: [i32; 2] },
}

#[derive(Debug, Clone)]
struct SheetAsset {
    width: u32,
    height: u32,
    rgba: Vec<u8>,
}

pub struct AssetManager {
    sprite_refs: HashMap<String, SpriteRef>,
    loaded_sprites: HashMap<String, SpriteAsset>,
    loaded_sheets: HashMap<PathBuf, SheetAsset>,
}

impl AssetManager {
    pub fn from_loaded_definitions(defs: LoadedDefinitions) -> Self {
        let mut sprite_refs = HashMap::new();

        for s in defs.sprites {
            let key = s.key;
            let sref = match s.sprite_source {
                SpriteSource::Standalone { file_path } => SpriteRef::Standalone { path: file_path },
                SpriteSource::Sheet { sheet_path, offset, size } => {
                    SpriteRef::SheetRegion { sheet_path, offset, size }
                }
            };

            sprite_refs.insert(key, sref);
        }

        Self {
            sprite_refs,
            loaded_sprites: HashMap::new(),
            loaded_sheets: HashMap::new(),
        }
    }

    pub fn has_sprite(&self, key: &str) -> bool {
        self.sprite_refs.contains_key(key)
    }

    pub fn get_sprite(&mut self, key: &str) -> AssetResult<&SpriteAsset> {
        if !self.loaded_sprites.contains_key(key) {
            let sref = self.sprite_refs.get(key).cloned().ok_or_else(|| {
                AssetError::UnknownSprite { key: key.to_string() }
            })?;

            let sprite = match sref {
                SpriteRef::Standalone { path } => load_sprite_rgba(&path)?,

                SpriteRef::SheetRegion { sheet_path, offset, size } => {
                    let sheet = self.load_sheet(&sheet_path)?;
                    extract_region_rgba(sheet, offset, size)?
                }
            };

            self.loaded_sprites.insert(key.to_string(), sprite);
        }

        Ok(self.loaded_sprites.get(key).expect("sprite inserted"))
    }

    fn load_sheet(&mut self, sheet_path: &Path) -> AssetResult<&SheetAsset> {
        if !self.loaded_sheets.contains_key(sheet_path) {
            let sheet = load_sheet_rgba(sheet_path)?;
            self.loaded_sheets.insert(sheet_path.to_path_buf(), sheet);
        }
        Ok(self.loaded_sheets.get(sheet_path).expect("sheet inserted"))
    }
}

fn load_sprite_rgba(path: &Path) -> AssetResult<SpriteAsset> {
    let img = image::open(path).map_err(|e| AssetError::ImageDecode {
        path: path.to_path_buf(),
        source: e,
    })?;

    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let rgba = rgba_img.into_raw();

    Ok(SpriteAsset { width, height, rgba })
}

fn load_sheet_rgba(path: &Path) -> AssetResult<SheetAsset> {
    let img = image::open(path).map_err(|e| AssetError::ImageDecode {
        path: path.to_path_buf(),
        source: e,
    })?;

    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let rgba = rgba_img.into_raw();

    Ok(SheetAsset { width, height, rgba })
}

fn extract_region_rgba(sheet: &SheetAsset, offset: [i32; 2], size: [i32; 2]) -> AssetResult<SpriteAsset> {
    let (ox, oy) = (offset[0], offset[1]);
    let (w, h) = (size[0], size[1]);

    if ox < 0 || oy < 0 || w <= 0 || h <= 0 {
        return Err(AssetError::InvalidManifest(format!(
            "Invalid spritesheet region offset={offset:?} size={size:?}"
        )));
    }

    let ox = ox as u32;
    let oy = oy as u32;
    let w_u = w as u32;
    let h_u = h as u32;

    if ox.checked_add(w_u).unwrap_or(u32::MAX) > sheet.width ||
       oy.checked_add(h_u).unwrap_or(u32::MAX) > sheet.height {
        return Err(AssetError::InvalidManifest(format!(
            "Spritesheet region out of bounds: sheet={}x{}, offset={offset:?}, size={size:?}",
            sheet.width, sheet.height
        )));
    }

    // Crop: copy row-by-row (fast, predictable)
    let bytes_per_pixel = 4usize;
    let sheet_stride = (sheet.width as usize) * bytes_per_pixel;
    let region_stride = (w_u as usize) * bytes_per_pixel;

    let mut out = vec![0u8; (w_u as usize) * (h_u as usize) * bytes_per_pixel];

    for row in 0..(h_u as usize) {
        let src_y = (oy as usize) + row;
        let src_x = ox as usize;

        let src_start = src_y * sheet_stride + src_x * bytes_per_pixel;
        let src_end = src_start + region_stride;

        let dst_start = row * region_stride;
        let dst_end = dst_start + region_stride;

        out[dst_start..dst_end].copy_from_slice(&sheet.rgba[src_start..src_end]);
    }

    Ok(SpriteAsset {
        width: w_u,
        height: h_u,
        rgba: out,
    })
}