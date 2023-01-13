#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use once_cell::sync::OnceCell;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tauri::Config;

use crate::util::resolve;

mod command;
mod common;
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
    fn success(data: Option<T>) -> ApiResult<T> {
        ApiResult {
            code: 200,
            msg: "ok".to_string(),
            data,
        }
    }
    fn _error(msg: String) -> ApiResult<T> {
        ApiResult {
            code: 500,
            msg,
            data: None,
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::chapter::save_chapter,
            command::book::get_book_list,
            command::book::save_book,
            command::book::delete_book_by_id,
            command::chapter::get_chapter_page,
            command::chapter::get_chapter_list,
            command::chapter::find_chapter_by_id,
            command::chapter::create_chapter,
            command::attachment::add_file,
            command::env::get_env_info,
        ])
        .setup(|app| {
            resolve::resolve_setup(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
