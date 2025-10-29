use std::{fs, path::PathBuf};

use serde::de::DeserializeOwned;

pub fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let rmk_dir = config_dir.join("RyzenTuner");
    fs::create_dir_all(&rmk_dir).unwrap();
    rmk_dir
}

pub fn read_file_content(key: &str) -> Result<Option<String>, String> {
    let config_dir = config_dir();
    let config_path = config_dir.join(format!("{}.json", key));

    if !config_path.exists() {
        return Ok(None);
    }

    fs::read_to_string(config_path)
        .map(Some)
        .map_err(|e| e.to_string())
}

pub fn read_storage<T: DeserializeOwned>(key: &str) -> Result<Option<T>, String> {
    match read_file_content(key)? {
        Some(content) => {
            let data: T = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}