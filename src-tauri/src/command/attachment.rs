use sea_orm::{ActiveModelTrait, IntoActiveModel};
use tauri::command;

use entity::attachment;

use crate::{ApiResult, DB};

#[command]
pub async fn add_file(att: attachment::Model) -> ApiResult<attachment::Model> {
    let db = DB.get().unwrap();
    let am = att.into_active_model();
    let model = am.insert(db).await.unwrap();
    ApiResult::success(Some(model))
}
