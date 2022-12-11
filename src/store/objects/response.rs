use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Response<T>{
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub items: Vec<T>
}