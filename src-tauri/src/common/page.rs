use sea_orm::ItemsAndPagesNumber;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Page<T> {
    pub datas: Vec<T>,
    pub page_no: u64,
    pub page_size: u64,
    pub total: u64,
    pub pages: u64,
}

impl<T> Page<T> {
    pub fn new(datas: Vec<T>, req: PageReq, items_and_pages: ItemsAndPagesNumber) -> Page<T> {
        Page {
            datas,
            pages: items_and_pages.number_of_pages,
            total: items_and_pages.number_of_items,
            page_no: req.page_no + 1,
            page_size: req.page_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PageReq {
    pub page_no: u64,
    pub page_size: u64,
}

impl PageReq {
    pub fn page_fix(mut self) -> PageReq {
        self.page_no -= 1;
        self
    }
}
