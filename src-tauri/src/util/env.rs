use crate::common;
use common::env_info::EnvInfo;
use std::fs;
use std::path::PathBuf;

pub fn get_env_info() -> EnvInfo {
    EnvInfo::default()
}

pub fn get_home_dir_path() -> PathBuf {
    tauri::api::path::home_dir().unwrap()
}
pub fn get_config_dir_path() -> PathBuf {
    let path = get_home_dir_path().join(".config").join("ripple");
    if !path.exists() {
        fs::create_dir_all(&path).expect("create config dir error");
    }
    path
}

pub fn get_log_dir_path() -> PathBuf {
    let path = get_config_dir_path().join("log");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}

pub fn get_db_dir_path() -> PathBuf {
    let path = get_config_dir_path().join("db");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}

pub fn get_attachment_dir_dir() -> PathBuf {
    let path = get_config_dir_path().join("attachment");
    if !path.exists() {
        fs::create_dir_all(&path).expect("create att dir error");
    }
    path
}

pub fn get_cover_dir_dir() -> PathBuf {
    let path = get_config_dir_path().join("cover");
    if !path.exists() {
        fs::create_dir_all(&path).expect("create cover dir error");
    }
    path
}
