pub mod cmds;
pub mod types;

use crate::{cmds::set_apu_tuning_config, types::AppState};
use crate::types::ApuTuningConfig;
use ryzen_tuner_core::RyzenAdj;
use specta_typescript::Typescript;
use tauri::Manager;
use tauri::async_runtime::Mutex;
use tauri_specta::{collect_commands, Builder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建 Tauri Specta Builder
    let builder = Builder::<tauri::Wry>::new()
        .typ::<ApuTuningConfig>()
        .commands(collect_commands![set_apu_tuning_config]);

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
            app.manage(AppState {
                ryzenadj: Mutex::new(RyzenAdj::new().unwrap()),
            });
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}