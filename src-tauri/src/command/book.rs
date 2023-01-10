use crate::common::page::{Page, PageReq};
use crate::{ApiResult, DB};
use entity::book::Model as BookModel;
use entity::prelude::Book;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryOrder};

#[tauri::command]
pub async fn get_book_list(req: PageReq) -> ApiResult<Page<BookModel>> {
    let req = req.page_fix();
    let db = DB.get().expect("get db error");
    let paginator = Book::find()
        .order_by_desc(entity::book::Column::CreateTime)
        .paginate(db, req.page_size);
    let data_vec = paginator.fetch_page(req.page_no).await.unwrap();
    let item = paginator.num_items_and_pages().await.unwrap();
    let page = Page::new(data_vec, req, item);
    ApiResult::success(Some(page))
}

#[tauri::command]
pub async fn save_book(book: BookModel) -> ApiResult<BookModel> {
    let db = DB.get().unwrap();
    let active_model = book.into_active_model();
    let model = active_model.insert(db).await.unwrap();
    ApiResult::success(Some(model))
}
