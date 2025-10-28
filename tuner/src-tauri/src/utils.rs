use std::{fs, path::PathBuf};

pub fn config_dir() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let rmk_dir = config_dir.join("RyzenTuner");
    fs::create_dir_all(&rmk_dir).unwrap();
    rmk_dir
}
