use ryzen_tuner_core::RyzenAdj;
use specta::Type;
use tauri::async_runtime::Mutex;

#[derive(serde::Serialize, serde::Deserialize, Debug, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ApuTuningConfig {
    pub temperature_limit: Option<u32>,
    pub skin_temperature_limit: Option<u32>,
    pub stapm_power_limit: Option<u32>,
    pub slow_power_limit: Option<u32>,
    pub slow_boost_duration: Option<u32>,
    pub fast_power_limit: Option<u32>,
    pub fast_boost_duration: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProfilesState {
    pub profiles: Vec<(String, ApuTuningConfig)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AutoProfileNames {
    pub charge: Option<String>,
    pub discharge: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Type, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettingsState {
    pub auto_set_profile: bool,
    pub auto_profile_names: AutoProfileNames,
}

pub struct AppState {
    pub ryzenadj: Mutex<RyzenAdj>,
}