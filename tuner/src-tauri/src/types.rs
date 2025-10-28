use ryzen_tuner_core::RyzenAdj;
use specta::Type;
use tauri::async_runtime::Mutex;

#[derive(serde::Serialize, serde::Deserialize, Debug, Type)]
pub struct ApuTuningConfig {
    pub temperature_limit: Option<u32>,
    pub skin_temperature_limit: Option<u32>,
    pub stapm_power_limit: Option<u32>,
    pub slow_power_limit: Option<u32>,
    pub slow_boost_duration: Option<u32>,
    pub fast_power_limit: Option<u32>,
    pub fast_boost_duration: Option<u32>,
}

pub struct AppState {
    pub ryzenadj: Mutex<RyzenAdj>,
}
