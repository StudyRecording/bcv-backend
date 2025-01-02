use chrono::Local;
use entity::{book_info, prelude::BookInfo};
use pojo::book::BookInfoSaveParam;
use sea_orm::{
    entity::*, ActiveValue::NotSet, DatabaseConnection, PaginatorTrait, QueryOrder, Set,
};
use utils::{err::ResultErr, page::Page};

/// 新增
pub async fn save(db: &DatabaseConnection, book_info: BookInfoSaveParam) -> Result<bool, ResultErr> {
    let book_info = book_info::ActiveModel {
        id: NotSet,
        book_name: Set(book_info.book_name),
        cover: Set(book_info.cover),
        path: Set(book_info.path),
        status: NotSet,
        create_time: Set(Local::now().naive_local()),
        create_user: Set("system".into()),
        update_time: Set(Local::now().naive_local()),
        update_user: Set("system".into()),
    }
    .save(db)
    .await;

    match book_info {
        Ok(_) => Ok(true),
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}

/// 根据id进行查询
pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<book_info::Model, ResultErr> {
    let book_info = BookInfo::find_by_id(id).one(db).await;
    match book_info {
        Ok(b_info) => {
            if b_info.is_some() {
                return Ok(b_info.unwrap());
            } else {
                return Err(ResultErr::BizErr {
                    msg: "未找到相关数据".into(),
                });
            }
        }
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}

/// 分页查询
pub async fn page_list(
    db: &DatabaseConnection,
    curr_page: u64,
    page_size: u64,
) -> Result<Page<book_info::Model>, ResultErr> {
    assert!(curr_page > 0, "页码必须大于零");
    let paginate = BookInfo::find()
        .order_by_desc(book_info::Column::UpdateTime)
        .paginate(db, page_size);
    let num_pages = paginate.num_pages().await.unwrap_or(0);

    let curr_page = curr_page - 1;
    let data_res = paginate.fetch_page(curr_page).await;

    match data_res {
        Ok(data) => Ok(Page {
            data: Some(data),
            curr_page: curr_page + 1,
            page_size,
            num_pages,
        }),
        Err(e) => Err(ResultErr::BizErr { msg: e.to_string() }),
    }
}
