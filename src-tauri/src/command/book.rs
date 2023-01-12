use crate::common::page::{Page, PageReq};
use crate::{ApiResult, DB};
use entity::book::Model as BookModel;
use entity::prelude::Book;
use entity::{book, chapter};
use migration::Expr;
use sea_orm::{
    ActiveModelTrait, DatabaseBackend, EntityTrait, FromQueryResult, IntoActiveModel,
    PaginatorTrait, QueryOrder, QuerySelect, Statement,
};
use std::collections::HashMap;

#[tauri::command]
pub async fn get_book_list() -> ApiResult<Vec<BookModel>> {
    let db = DB.get().expect("get db error");
    let data_vec = Book::find()
        .order_by_desc(entity::book::Column::CreateTime)
        .all(db)
        .await
        .unwrap();
    ApiResult::success(Some(data_vec))
}

#[tauri::command]
pub async fn save_book(book: BookModel) -> ApiResult<BookModel> {
    let db = DB.get().unwrap();
    let active_model = book.into_active_model();
    let model = active_model.insert(db).await.unwrap();
    ApiResult::success(Some(model))
}

#[derive(FromQueryResult)]
struct BookTextCount {
    text_count: i32,
    book_id: String,
}

#[tauri::command]
pub async fn get_book_text_count() -> ApiResult<HashMap<String, i32>> {
    let db = DB.get().unwrap();
    let sql = r#"select sum(text_count) as text_count,book_id
                            from chapter group by book_id,book_name"#
        .to_string();
    let data_vec = chapter::Entity::find()
        .from_raw_sql(Statement::from_string(DatabaseBackend::Sqlite, sql))
        .into_model::<BookTextCount>()
        .all(db)
        .await;

    let mut map = HashMap::<String, i32>::new();

    if data_vec.is_ok() {
        let data_vec = data_vec.unwrap();
        for count in data_vec {
            map.insert(count.book_id, count.text_count);
        }
    }
    ApiResult::success(Some(map))
}
