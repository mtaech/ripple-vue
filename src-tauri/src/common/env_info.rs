use crate::util::env;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EnvInfo {
    pub home_dir_path: String,
    pub config_dir_path: String,
    pub log_dir_path: String,
    pub db_dir_path: String,
    pub att_dir_path: String,
    pub cover_dir_path: String,
}

impl EnvInfo {
    pub fn default() -> Self {
        EnvInfo {
            home_dir_path: path_to_string(env::get_home_dir_path()),
            config_dir_path: path_to_string(env::get_config_dir_path()),
            log_dir_path: path_to_string(env::get_db_dir_path()),
            db_dir_path: path_to_string(env::get_db_dir_path()),
            att_dir_path: path_to_string(env::get_attachment_dir_dir()),
            cover_dir_path: path_to_string(env::get_cover_dir_path()),
        }
    }
}

fn path_to_string(path: PathBuf) -> String {
    path.into_os_string().into_string().unwrap()
}
