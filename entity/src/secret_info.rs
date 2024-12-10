//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use chrono::{Local, DateTime};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "secret_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub access_secret: String,
    pub refresh_secret: String,
    pub access_end_time: Option<DateTime<Local>>,
    pub refresh_end_time: Option<DateTime<Local>>,
    pub status: i8,
    pub create_time: DateTime<Local>,
    pub create_user: String,
    pub update_time: DateTime<Local>,
    pub update_user: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}