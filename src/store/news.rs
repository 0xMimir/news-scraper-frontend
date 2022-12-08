use super::objects::news::{NewsSource, Response};
use crate::helpers::{error::Error, request::request};
use reqwest::Method;

pub struct NewsStore;

impl NewsStore {
    pub async fn get_news_sources() -> Result<Vec<NewsSource>, Error> {
        request(Method::GET, "/public/sources", None::<()>).await
    }
    pub async fn get_news_count() -> Result<i32, Error> {
        Self::get_news_sources().await.map(|s| s.len() as i32)
    }
    pub async fn get_news() -> Result<Response, Error> {
        request(Method::GET, "/public/example", None::<()>).await
    }
}
