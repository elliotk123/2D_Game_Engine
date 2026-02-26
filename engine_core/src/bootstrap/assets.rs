use asset_manager::{AssetManager, AssetResult, load_manifest_and_definitions};

pub fn build_asset_manager(manifest_path: &str) -> AssetResult<AssetManager> {
    let defs = load_manifest_and_definitions(manifest_path)?;
    Ok(AssetManager::from_loaded_definitions(defs))
}