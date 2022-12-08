use serde::Deserialize;

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