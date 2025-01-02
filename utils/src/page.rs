use derive_more::derive::Debug;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub data: Option<Vec<T>>,
    pub curr_page: u64,
    pub page_size: u64,
    pub num_pages: u64,
}
