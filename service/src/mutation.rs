use ::entity::post;
use sea_orm::{DbConn, DbErr, Set};
use sea_orm::*;

pub struct Mutation;


impl Mutation {
    
     pub async fn create_post(db: &DbConn, form_data: post::Model) -> Result<post::ActiveModel, DbErr> {
        post::ActiveModel {
            title: Set(form_data.title.to_owned()),
            text: Set(form_data.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }

}