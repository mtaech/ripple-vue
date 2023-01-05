use crate::{ApiResult, DB};
use entity::chapter;
use entity::prelude::Character;
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel};

#[tauri::command(async)]
pub async fn save_chapter(chapter: chapter::Model) -> ApiResult<chapter::Model> {
    let db = DB.get().unwrap();
    let model = chapter.into_active_model();
    let result = model.insert(db).await.unwrap();
    ApiResult::success("ok".to_string(), Some(result))
}
