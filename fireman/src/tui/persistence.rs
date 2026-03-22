use crate::model::OptimizationStore;
use std::{fs, path::PathBuf};

use super::types::OptimizationStoreEnvelope;

pub(crate) fn optimization_store_path() -> Result<PathBuf, String> {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return Ok(PathBuf::from(config_home).join("firebat/settings.json"));
    }
    let home = std::env::var("HOME").map_err(|error| error.to_string())?;
    Ok(PathBuf::from(home).join(".config/firebat/settings.json"))
}

pub(crate) fn load_optimization_store() -> Result<OptimizationStore, String> {
    let path = optimization_store_path()?;
    if !path.exists() {
        return Err(format!("settings file not found at {}", path.display()));
    }
    let json = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&json)
        .or_else(|_| {
            serde_json::from_str::<OptimizationStoreEnvelope>(&json)
                .map(|config| config.optimization_store)
        })
        .map_err(|error| error.to_string())
}

pub(crate) fn save_optimization_store(store: &OptimizationStore) -> Result<(), String> {
    let path = optimization_store_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let json = serde_json::to_string_pretty(store).map_err(|error| error.to_string())?;
    fs::write(path, json).map_err(|error| error.to_string())
}
