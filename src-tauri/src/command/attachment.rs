use log::info;
use nanoid::nanoid;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, IntoActiveModel};
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use tauri::command;

use entity::attachment;
use entity::prelude::Attachment;

use crate::util::env::{get_config_dir_path, get_cover_dir_path};
use crate::util::file;
use crate::{ApiResult, DB};

#[command]
pub async fn add_file(att: attachment::Model) -> ApiResult<attachment::Model> {
    let db = DB.get().unwrap();
    let am = att.into_active_model();
    let model = am.insert(db).await.unwrap();
    ApiResult::success(Some(model))
}

pub async fn save_cover_from_url(cover_url: String) -> attachment::Model {
    info!("cover url:{}", cover_url);
    let db = DB.get().unwrap();
    let cover_path = PathBuf::from(cover_url);
    let mut model = attachment::ActiveModel::new();
    let metadata = fs::metadata(&cover_path).unwrap();
    let (content_type, suffix) = file::get_content_type(cover_path.clone());
    let path = copy_cover(&cover_path, model.id.as_ref(), &suffix);
    let len = metadata.len();
    let file_name = cover_path
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
    model.suffix = Set(Some(suffix));
    model.content_type = Set(Some(content_type));
    model.file_size = Set(Some(len as i32));
    model.file_name = Set(Some(file_name));
    model.file_path = Set(Some(path));
    let model = model.insert(db).await.unwrap();
    model
}

fn copy_cover(path: &PathBuf, id: &str, suffix: &str) -> String {
    let to_path = get_cover_dir_path()
        .join(format!("{}.{}", id, suffix))
        .into_os_string()
        .into_string()
        .unwrap();
    fs::copy(path, &to_path).unwrap();
    to_path
}
