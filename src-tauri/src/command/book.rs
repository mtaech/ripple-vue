use crate::command::attachment;
use crate::common::page::{Page, PageReq};
use crate::{ApiResult, DB};
use entity::book::{Model as BookModel, Model};
use entity::prelude::Book;
use entity::{book, chapter};
use log::info;
use migration::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseBackend, DeleteResult, EntityTrait, FromQueryResult,
    IntoActiveModel, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Statement,
};
use serde_json::map::Values;
use serde_json::{json, Value};
use std::collections::HashMap;
use tauri_utils::assets::phf::Map;

#[tauri::command]
pub async fn get_book_list() -> ApiResult<Vec<BookModel>> {
    let db = DB.get().expect("get db error");
    let data_vec = Book::find()
        .order_by_desc(entity::book::Column::CreateTime)
        .all(db)
        .await
        .unwrap();
    let map = get_book_text_count().await;
    let data_vec = data_vec
        .into_iter()
        .map(|mut data| {
            let count = map.get(&data.id).unwrap().clone();
            data.text_count = Some(count);
            data
        })
        .collect::<Vec<Model>>();

    ApiResult::success(Some(data_vec))
}

#[tauri::command]
pub async fn save_book(book: BookModel, cover_url: String) -> ApiResult<BookModel> {
    let db = DB.get().unwrap();
    let mut active_model = book.into_active_model();
    let cover_model = attachment::save_cover_from_url(cover_url).await;
    active_model.cover_path = Set(cover_model.file_path);
    active_model.cover_id = Set(Some(cover_model.id));
    let model = active_model.insert(db).await.unwrap();
    ApiResult::success(Some(model))
}

#[derive(FromQueryResult, Debug)]
struct BookTextCount {
    text_count: Option<i32>,
    book_id: String,
}

pub async fn get_book_text_count() -> HashMap<String, i32> {
    let db = DB.get().unwrap();

    let sql = r#"select sum(case c.text_count when c.text_count is null 
                                            then 0 else c.text_count end) as 'text_count', b.id as 'book_id'
                            from book b left join chapter c on b.id = c.book_id group by b.id "#
        .to_string();
    let data_vec = chapter::Entity::find()
        .from_raw_sql(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .into_model::<BookTextCount>()
        .all(db)
        .await;
    let mut map = HashMap::<String, i32>::new();
    info!("vec data:{:#?}", data_vec);

    if data_vec.is_ok() {
        let data_vec = data_vec.unwrap();
        for count in data_vec {
            let mut text_count = 0;
            if count.text_count.is_some() {
                text_count = count.text_count.unwrap();
            }
            map.insert(count.book_id, text_count);
        }
    }
    map
}

#[tauri::command]
pub async fn delete_book_by_id(book_id: String) -> ApiResult<u64> {
    let db = DB.get().unwrap();
    let result = book::Entity::delete_by_id(book_id).exec(db).await.unwrap();
    ApiResult::success(Some(result.rows_affected))
}
