use std::path::PathBuf;

pub type AssetResult<T> = Result<T, AssetError>;

#[derive(Debug)]
pub enum AssetError {
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    TomlParse {
        path: PathBuf,
        source: toml::de::Error,
    },
    ImageDecode {
        path: PathBuf,
        source: image::ImageError,
    },

    // Asset lookup/runtime
    UnknownSprite {
        key: String,
    },

    // Validation/contract
    DuplicateKey {
        asset_type: &'static str,
        key: String,
        first_source: PathBuf,
        second_source: PathBuf,
    },
    MissingSpriteFilePath {
        key: String,
        source: PathBuf,
    },
    AnimationReferencesMissingSprite {
        animation_key: String,
        missing_sprite_key: String,
        source: PathBuf,
    },

    InvalidManifest(String),
}