pub mod error;
pub mod manifest;
pub mod sprites;

pub use error::{AssetError, AssetResult};

// Manifest/root types
pub use manifest::{
    AssetDefinitions,
    AudioDefinition,
    ManifestRoot,
    SpriteDefinition,
    SpriteSheetDefinition,
    AnimationDefinition,
    LoadedDefinitions,
};

// Runtime-facing manager types
pub use sprites::{AssetManager, SpriteAsset};

pub use manifest::load_manifest_and_definitions;
pub use manifest::{ResolvedAudio, ResolvedSprite, SpriteSource};