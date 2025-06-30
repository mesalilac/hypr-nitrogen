use crate::APP_NAME;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn get_cache_dir() -> PathBuf {
    let cache_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .cache_dir()
        .join(APP_NAME);

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).expect("Failed to create cache dir");
    }

    cache_dir
}

pub fn get_app_data_dir() -> PathBuf {
    let app_data_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .data_local_dir()
        .join(APP_NAME);

    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir).expect("Failed to create data dir");
    }

    app_data_dir
}

#[allow(dead_code)]
pub fn get_config_dir() -> PathBuf {
    let config_dir = directories::BaseDirs::new()
        .expect("Failed to get base dir")
        .config_dir()
        .join(APP_NAME);

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Failed to create config dir");
    }

    config_dir
}
