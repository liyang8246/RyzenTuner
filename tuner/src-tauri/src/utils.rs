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

// APU Tuning utilities
use crate::types::{AppState, ApuTuningConfig};

macro_rules! set_optional_value {
    ($ryzenadj:expr, $config_field:expr, $setter:ident, $error_message:expr) => {
        if let Some(value) = $config_field {
            if let Err(e) = $ryzenadj.$setter(value) {
                log::error!("{}: {}", $error_message, e);
            }
        }
    };
}

pub async fn apply_apu_tuning_config(config: ApuTuningConfig, state: &tauri::State<'_, AppState>) -> Result<(), String> {
    let ryzenadj = state.ryzenadj.lock().await;
    set_optional_value!(
        ryzenadj,
        config.temperature_limit,
        set_tctl_temp,
        "Failed to set temperature limit"
    );
    set_optional_value!(
        ryzenadj,
        config.skin_temperature_limit,
        set_apu_skin_temp_limit,
        "Failed to set skin temperature limit"
    );
    set_optional_value!(
        ryzenadj,
        config.stapm_power_limit.map(|x| x * 1000),
        set_stapm_limit,
        "Failed to set STAPM power limit"
    );
    set_optional_value!(
        ryzenadj,
        config.slow_power_limit.map(|x| x * 1000),
        set_slow_limit,
        "Failed to set slow power limit"
    );
    set_optional_value!(
        ryzenadj,
        config.slow_boost_duration,
        set_slow_time,
        "Failed to set slow boost duration"
    );
    set_optional_value!(
        ryzenadj,
        config.fast_power_limit.map(|x| x * 1000),
        set_fast_limit,
        "Failed to set fast power limit"
    );
    set_optional_value!(
        ryzenadj,
        config.fast_boost_duration,
        set_stapm_time,
        "Failed to set fast boost duration"
    );
    Ok(())
}