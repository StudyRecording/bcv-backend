use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder,
};
use entity::book_info;
use pojo::book::BookInfoSaveParam;
use service::{book, AppState};
use utils::{err::ResultErr, page::Page, res::ResultRes};

/// 保存数据
#[post("/save")]
pub async fn save(
    info: Json<BookInfoSaveParam>,
    data: Data<AppState>,
) -> Result<impl Responder, ResultErr> {
    let book_info = info.into_inner();

    let save_result = service::book::save(&data.conn, book_info).await;
    match save_result {
        Ok(b) => Ok(ResultRes::success(b)),
        Err(_) => Err(ResultErr::BizErr {
            msg: "保存失败".into(),
        }),
    }
}

/// 获取详情
#[get("/get/{id}")]
pub async fn get_by_id(
    id: actix_web::web::Path<i32>,
    data: Data<AppState>,
) -> Result<ResultRes<book_info::Model>, ResultErr> {
    let id = id.into_inner();

    let book_info = book::get_by_id(&data.conn, id).await?;
    Ok(ResultRes::success(book_info))
}

/// 分页查询
#[get("/page/{curr_page}/{page_size}")]
pub async fn page_list(
    path: Path<(u64, u64)>,
    data: Data<AppState>,
) -> Result<ResultRes<Page<book_info::Model>>, ResultErr> {
    let (curr_page, page_size) = path.into_inner();

    let page_list = book::page_list(&data.conn, curr_page, page_size).await;
    match page_list {
        Ok(record) => Ok(ResultRes::success(record)),
        Err(_) => Err(ResultErr::BizErr {
            msg: "列表查询失败".into(),
        }),
    }
}
