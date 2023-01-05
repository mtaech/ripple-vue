#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tauri::Config;

use crate::util::init::init_db;
use crate::util::{init, resolve};

mod command;
mod db;
mod util;
static DB: OnceCell<DatabaseConnection> = OnceCell::new();

#[derive(Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub code: u32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResult<T> {
    fn success(msg: String, data: Option<T>) -> ApiResult<T> {
        ApiResult {
            code: 200,
            msg,
            data,
        }
    }
    fn error(msg: String) -> ApiResult<T> {
        ApiResult {
            code: 500,
            msg,
            data: None,
        }
    }
}

fn main() {
    let config = Config::default();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::chapter::save_chapter])
        .setup(|app| Ok(resolve::resolve_setup(app)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
