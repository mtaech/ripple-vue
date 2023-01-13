use chrono::Local;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Default, Serialize, Deserialize)]
#[sea_orm(table_name = "book")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: Option<String>,
    pub cover_id: Option<String>,
    pub cover_path: Option<String>,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub modified_time: Option<String>,
    #[sea_orm(ignore)]
    pub text_count: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn before_save(mut self, insert: bool) -> Result<Self, DbErr> {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        if insert {
            self.create_time = Set(Some(time));
        } else {
            self.modified_time = Set(Some(time));
        }
        Ok(self)
    }
}
