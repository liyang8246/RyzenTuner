use crate::types::{AppState, ApuTuningConfig, ProfilesState, SettingsState};
use crate::utils::{read_storage, apply_apu_tuning_config};
use std::collections::HashMap;
use std::time::Duration;
use systemstat::{Platform, System};
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
        let mut last_ac_power_state: Option<bool> = None;
        let sys = System::new();

        loop {
            tokio::time::sleep(Duration::from_secs(5)).await;

            let Ok(Some(settings)) = read_storage::<SettingsState>("pinia-setting") else {
                continue;
            };
            if !settings.auto_set_profile {
                continue;
            }

            let ac_power_status = match sys.on_ac_power() {
                Ok(status) => status,
                Err(e) => {
                    log::warn!("Failed to get AC power status: {}", e);
                    continue;
                }
            };

            // Only proceed if AC power status changed
            if last_ac_power_state == Some(ac_power_status) {
                continue;
            }
            last_ac_power_state = Some(ac_power_status);

            let profiles = read_storage::<ProfilesState>("pinia-profiles")
                .unwrap()
                .unwrap_or_else(|| ProfilesState { profiles: Vec::new() })
                .profiles;
            let profiles: HashMap<String, ApuTuningConfig> = profiles.into_iter().collect();

            let profile_name = if ac_power_status {
                &settings.auto_profile_names.charge
            } else {
                &settings.auto_profile_names.discharge
            };

            if let Some(profile_name) = profile_name {
                if let Some(config) = profiles.get(profile_name) {
                    log::info!("Applying '{}' profile due to AC power state: {}", profile_name, ac_power_status);
                    if let Err(e) = apply_apu_tuning_config(config.clone(), &app_handle.state::<AppState>()).await {
                        log::error!("Failed to apply tuning config: {}", e);
                    }
                } else {
                    log::warn!("Profile '{}' not found, skipping auto configuration", profile_name);
                }
            } else {
                log::warn!("No profile configured for {} state, skipping auto configuration", if ac_power_status { "charging" } else { "discharging" });
            }
        }
    });
    Ok(())
}
