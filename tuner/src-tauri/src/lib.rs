use ryzen_tuner_core::RyzenAdj;
use specta::{Type, specta};
use specta_typescript::Typescript;
use std::sync::Mutex;
use tauri::Manager;
use tauri_specta::{Builder, collect_commands};

struct AppState {
    ryzenadj: Mutex<RyzenAdj>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Type)]
struct ApuTuningConfig {
    temperature_limit: Option<u32>,
    skin_temperature_limit: Option<u32>,
    stapm_power_limit: Option<u32>,
    slow_power_limit: Option<u32>,
    slow_boost_duration: Option<u32>,
    fast_power_limit: Option<u32>,
    fast_boost_duration: Option<u32>,
}

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
fn set_apu_tuning_config(config: ApuTuningConfig, state: tauri::State<AppState>) {
    let ryzenadj = state.ryzenadj.lock().unwrap();

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
}

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
            // 初始化日志插件
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

            // 挂载 Tauri Specta 事件
            builder.mount_events(app);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
