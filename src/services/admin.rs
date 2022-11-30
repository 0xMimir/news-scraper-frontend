use reqwest::Method;
use serde::Deserialize;

use crate::Error;

use super::helpers::request;

pub struct AdminStore;

impl AdminStore{
    pub async fn get_scraper_info() -> Result<Vec<ScraperInfo>, Error>{
        request(Method::GET, "/admin/scrapers", None::<()>).await
    }
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct ScraperInfo{
    pub blog_id: String,
    pub total: i32,
    pub scraped: i32,
    pub unscraped: i32,
    pub deleted: i32,
    pub processed: i32,
    pub error: i32
}