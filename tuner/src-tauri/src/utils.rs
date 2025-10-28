use std::{fs, path::PathBuf};

pub fn config_file() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let rmk_dir = config_dir.join("RyzenTuner");
    fs::create_dir_all(&rmk_dir).unwrap();
    let config_file_path = rmk_dir.join("config.toml");
    if !config_file_path.exists() {
        fs::File::create(&config_file_path).unwrap();
    }
    config_file_path
}
