use std::{fs, path::PathBuf};

use crate::error::{Result, RyzenTunerError};
use serde::de::DeserializeOwned;

pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| RyzenTunerError::Config("Failed to get config directory".to_string()))?;
    let rmk_dir = config_dir.join("RyzenTuner");
    fs::create_dir_all(&rmk_dir)?;
    Ok(rmk_dir)
}

pub fn read_file_content(key: &str) -> Result<Option<String>> {
    let config_dir = config_dir()?;
    let config_path = config_dir.join(format!("{}.json", key));

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(config_path)?;
    Ok(Some(content))
}

pub fn read_storage<T: DeserializeOwned>(key: &str) -> Result<Option<T>> {
    match read_file_content(key)? {
        Some(content) => {
            let data: T = serde_json::from_str(&content)?;
            Ok(Some(data))
        }
        None => Ok(None),
    }
}

// APU Tuning utilities
use crate::types::{AppState, ApuTuningConfig};

pub async fn apply_apu_tuning_config(config: ApuTuningConfig, state: &tauri::State<'_, AppState>) -> Result<()> {
    let ryzenadj = state.ryzenadj.lock().await;

    if let Some(value) = config.temperature_limit {
        ryzenadj.set_tctl_temp(value)?;
    }
    if let Some(value) = config.skin_temperature_limit {
        ryzenadj.set_apu_skin_temp_limit(value)?;
    }
    if let Some(value) = config.stapm_power_limit.map(|x| x * 1000) {
        ryzenadj.set_stapm_limit(value)?;
    }
    if let Some(value) = config.slow_power_limit.map(|x| x * 1000) {
        ryzenadj.set_slow_limit(value)?;
    }
    if let Some(value) = config.slow_boost_duration {
        ryzenadj.set_slow_time(value)?;
    }
    if let Some(value) = config.fast_power_limit.map(|x| x * 1000) {
        ryzenadj.set_fast_limit(value)?;
    }
    if let Some(value) = config.fast_boost_duration {
        ryzenadj.set_stapm_time(value)?;
    }

    Ok(())
}