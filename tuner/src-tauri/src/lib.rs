pub mod cmds;
pub mod plugins;
pub mod types;
pub mod utils;

use crate::cmds::*;
use crate::plugins::{setup_logging_plugin, setup_scheduler_plugin, setup_tray_icon};
use crate::types::{AppState, ApuTuningConfig, ProfilesState, SettingsState};
use ryzen_tuner_core::RyzenAdj;
#[cfg(debug_assertions)]
use specta_typescript::Typescript;
use tauri::Manager;
use tauri::async_runtime::Mutex;
use tauri_specta::{Builder, collect_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = Builder::<tauri::Wry>::new()
        .typ::<ApuTuningConfig>()
        .typ::<SettingsState>()
        .typ::<ProfilesState>()
        .commands(collect_commands![
            set_apu_tuning_config,
            storage_read,
            storage_write,
            hide_window,
            show_window
        ]);

    #[cfg(debug_assertions)]
    specta_builder
        .export(Typescript::default(), "../app/types/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            setup_logging_plugin(&app.handle())?;
            setup_tray_icon(&app.handle())?;
            setup_scheduler_plugin(&app.handle())?;
            app.manage(AppState {
                ryzenadj: Mutex::new(RyzenAdj::new().unwrap()),
            });
            specta_builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
