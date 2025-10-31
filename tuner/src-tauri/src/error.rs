use thiserror::Error;

pub type Result<T> = std::result::Result<T, RyzenTunerError>;

#[derive(Error, Debug)]
pub enum RyzenTunerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("RyzenAdj error: {0}")]
    RyzenAdj(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("System error: {0}")]
    System(String),

    #[error("Window operation error: {0}")]
    Window(String),

    #[error("Plugin setup error: {0}")]
    Plugin(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("Failed to apply tuning settings")]
    TuningFailed,
}

impl From<tauri::Error> for RyzenTunerError {
    fn from(err: tauri::Error) -> Self {
        RyzenTunerError::Window(err.to_string())
    }
}

// systemstat doesn't expose a PlatformError type, so we'll handle it differently
// This conversion will be done manually where needed

// For RyzenAdj errors from ryzen-tuner-core
impl From<()> for RyzenTunerError {
    fn from(_: ()) -> Self {
        RyzenTunerError::RyzenAdj("Failed to initialize RyzenAdj".to_string())
    }
}

impl From<i32> for RyzenTunerError {
    fn from(code: i32) -> Self {
        RyzenTunerError::RyzenAdj(format!("RyzenAdj error code: {}", code))
    }
}