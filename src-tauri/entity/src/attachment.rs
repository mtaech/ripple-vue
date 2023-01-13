//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use chrono::Local;
use nanoid::nanoid;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Default, Serialize, Deserialize)]
#[sea_orm(table_name = "attachment")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub file_name: Option<String>,
    pub file_size: Option<i32>,
    pub file_path: Option<String>,
    pub suffix: Option<String>,
    pub content_type: Option<String>,
    pub create_time: Option<String>,
    pub modified_time: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(nanoid!(10)),
            ..ActiveModelTrait::default()
        }
    }
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
