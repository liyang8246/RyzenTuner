pub mod cmds;
pub mod types;
pub mod utils;

use crate::types::ApuTuningType;
use crate::{cmds::*, types::AppState};
use ryzen_tuner_core::RyzenAdj;
use specta_typescript::Typescript;
use tauri::Manager;
use tauri::async_runtime::Mutex;
use tauri::tray::TrayIconBuilder;
use tauri_specta::{Builder, collect_commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new()
        .typ::<ApuTuningType>()
        .commands(collect_commands![set_apu_tuning_config, storage_read, storage_write]);

    #[cfg(debug_assertions)]
    builder
        .export(Typescript::default(), "../app/types/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .build(app)?;

            app.manage(AppState {
                ryzenadj: Mutex::new(RyzenAdj::new().unwrap()),
            });
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
