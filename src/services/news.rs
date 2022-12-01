use reqwest::Method;
use serde::Deserialize;

use crate::Error;

use super::helpers::request;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewsSource{
    pub blog_url: String,
    pub blog_name: String,
    pub blog_id: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Response{
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub items: Vec<NewsEntry>
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NewsEntry{
    pub url: String,
    pub title: String,
    pub image: Option<String>,
    pub released_at_unix: Option<i64>,
    pub description: String,
    pub blog_name: String,
    pub blog_id: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Sentiment{
    Positive,
    Negative,
    Neutral
}

pub struct NewsStore;

impl NewsStore{
    pub async fn get_news_sources() -> Result<Vec<NewsSource>, Error>{
        request(
            Method::GET,
            "/public/sources",
            None::<()>
        ).await
    }
    pub async fn get_news_count() -> Result<i32, Error>{
        Self::get_news_sources().await
            .map(|s| s.len() as i32)
    }
    pub async fn get_news() -> Result<Response, Error>{
        request(
            Method::GET,
            "/public/example",
            None::<()>
        ).await
    }
}

