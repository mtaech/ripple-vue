use crate::common::page::{Page, PageReq};
use crate::{ApiResult, DB};
use entity::prelude::Chapter;
use entity::{book, chapter};
use log::info;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel,
    PaginatorTrait, QueryFilter, QueryOrder,
};

#[tauri::command]
pub async fn create_chapter() -> ApiResult<chapter::Model> {
    let db = DB.get().unwrap();
    let model = chapter::ActiveModel::new();
    let result = model.insert(db).await.unwrap();
    ApiResult::success(Some(result))
}

#[tauri::command]
pub async fn save_chapter(chapter: chapter::Model) -> ApiResult<chapter::Model> {
    let db = DB.get().unwrap();
    let chapter_opt = chapter::Entity::find_by_id(chapter.id.clone())
        .one(db)
        .await
        .unwrap();
    info!("opt:{:?}", chapter_opt);
    if chapter_opt.is_some() {
        let chapter_model = chapter_opt.unwrap();
        let mut am = chapter_model.clone().into_active_model();
        am.name = Set(chapter.name);
        am.text_content = Set(chapter.text_content);
        am.html_content = Set(chapter.html_content);
        am.text_count = Set(chapter.text_count);
        let model = am.update(db).await.unwrap();
        return ApiResult::success(Some(model));
    }
    let model = chapter.into_active_model();
    let result = model.insert(db).await.unwrap();
    ApiResult::success(Some(result))
}

#[tauri::command]
pub async fn get_chapter_page(req: PageReq, book_id: String) -> ApiResult<Page<chapter::Model>> {
    info!("req:{:?} ,book:{:?}", req, book_id);
    let req = req.page_fix();
    let db = DB.get().expect("get db error");
    let paginator = Chapter::find()
        .filter(chapter::Column::BookId.eq(book_id))
        .order_by_desc(chapter::Column::CreateTime)
        .paginate(db, req.page_size);
    let data_vec = paginator.fetch_page(req.page_no).await.unwrap();
    let item = paginator.num_items_and_pages().await.unwrap();
    let page = Page::new(data_vec, req, item);
    ApiResult::success(Some(page))
}

#[tauri::command]
pub async fn get_chapter_list(book_id: String) -> ApiResult<Vec<chapter::Model>> {
    let db = DB.get().expect("get db error");
    let model_vec = Chapter::find()
        .filter(chapter::Column::BookId.eq(book_id))
        .order_by_desc(chapter::Column::CreateTime)
        .all(db)
        .await
        .unwrap();
    ApiResult::success(Some(model_vec))
}

#[tauri::command]
pub async fn find_chapter_by_id(chapter_id: String, book_id: String) -> ApiResult<chapter::Model> {
    let db = DB.get().unwrap();
    let option = Chapter::find()
        .filter(chapter::Column::BookId.eq(book_id.clone()))
        .filter(chapter::Column::Id.eq(chapter_id.clone()))
        .one(db)
        .await
        .unwrap();
    // let option = Chapter::find_by_id(chapter_id).one(db).await.unwrap();
    if option.is_none() {
        let book_opt = book::Entity::find_by_id(book_id.clone())
            .one(db)
            .await
            .unwrap();
        if book_opt.is_some() {
            let book = book_opt.unwrap();
            let model = chapter::Model {
                id: chapter_id.clone(),
                book_id: Some(book.id),
                book_name: book.name,
                ..chapter::Model::default()
            };
            return ApiResult::success(Some(model));
        }
    }
    ApiResult::success(option)
}
