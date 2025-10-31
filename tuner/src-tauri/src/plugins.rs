use crate::types::{AppState, ApuTuningConfig, ProfilesState, SettingsState};
use crate::utils::read_storage;
use std::collections::HashMap;
use std::time::Duration;
use tauri::tray::TrayIconBuilder;
use tauri::{AppHandle, Manager, Result};

pub fn setup_logging_plugin(app_handle: &AppHandle) -> Result<()> {
    app_handle.plugin(
        tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
    )?;
    Ok(())
}

pub fn setup_tray_icon(app_handle: &AppHandle) -> Result<()> {
    let _tray = TrayIconBuilder::new()
        .icon(app_handle.default_window_icon().unwrap().clone())
        .build(app_handle)?;
    Ok(())
}

pub fn setup_scheduler_plugin(app_handle: &AppHandle) -> Result<()> {
    let app_handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let Ok(Some(settings)) = read_storage::<SettingsState>("pinia-setting") else {
                continue;
            };
            if !settings.auto_set_profile {
                continue;
            }
            let profiles = read_storage::<ProfilesState>("pinia-profiles").unwrap().unwrap().profiles;
            let profiles: HashMap<String, ApuTuningConfig> = profiles.into_iter().collect();

        }
    });
    Ok(())
}
