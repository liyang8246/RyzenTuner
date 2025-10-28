use std::fs;

use crate::{
    types::{AppState, ApuTuningConfig},
    utils::config_dir,
};
use specta::specta;

macro_rules! set_optional_value {
    ($ryzenadj:expr, $config_field:expr, $setter:ident, $error_message:expr) => {
        if let Some(value) = $config_field {
            if let Err(e) = $ryzenadj.$setter(value) {
                log::error!("{}: {}", $error_message, e);
            }
        }
    };
}

#[tauri::command]
#[specta]
pub async fn set_apu_tuning_config(config: ApuTuningConfig, state: tauri::State<'_, AppState>) -> Result<(), String> {
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
        config.stapm_power_limit,
        set_stapm_limit,
        "Failed to set STAPM power limit"
    );
    set_optional_value!(
        ryzenadj,
        config.slow_power_limit,
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
        config.fast_power_limit,
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

#[tauri::command]
#[specta]
pub async fn storage_read(key: String) -> Result<Option<String>, String> {
    let config_dir = config_dir();
    let config_path = config_dir.join(format!("{}.toml", key));

    if !config_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    let toml_value: toml::Value = toml::from_str(&content).map_err(|e| e.to_string())?;
    let json_string = serde_json::to_string(&toml_value).map_err(|e| e.to_string())?;

    Ok(Some(json_string))
}

#[tauri::command]
#[specta]
pub async fn storage_write(key: String, value: String) -> Result<(), String> {
    let config_dir = config_dir();
    let config_path = config_dir.join(format!("{}.toml", key));

    let json_value: serde_json::Value = serde_json::from_str(&value).map_err(|e| e.to_string())?;
    let toml_string = toml::to_string(&json_value).map_err(|e| e.to_string())?;

    fs::write(config_path, toml_string).map_err(|e| e.to_string())?;
    Ok(())
}
