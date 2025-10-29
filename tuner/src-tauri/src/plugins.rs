use tauri::{AppHandle, Result};
use tauri::tray::TrayIconBuilder;

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
