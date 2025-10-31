use std::fs;

use crate::{
    types::{AppState, ApuTuningConfig},
    utils::{config_dir, read_file_content, apply_apu_tuning_config},
};
use specta::specta;
use tauri::Window;

#[tauri::command]
#[specta]
pub async fn set_apu_tuning_config(config: ApuTuningConfig, state: tauri::State<'_, AppState>) -> std::result::Result<(), String> {
    apply_apu_tuning_config(config, &state).await.map_err(|e| e.to_string())
}

#[tauri::command]
#[specta]
pub async fn storage_read(key: String) -> std::result::Result<Option<String>, String> {
    read_file_content(&key).map_err(|e| e.to_string())
}

#[tauri::command]
#[specta]
pub async fn storage_write(key: String, value: String) -> std::result::Result<(), String> {
    let config_dir = config_dir().map_err(|e| e.to_string())?;
    let config_path = config_dir.join(format!("{}.json", key));

    fs::write(config_path, value).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
#[specta]
pub fn hide_window(window: Window) -> std::result::Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
#[specta]
pub fn show_window(window: Window) -> std::result::Result<(), String> {
    window.show().map_err(|e| e.to_string())
}
