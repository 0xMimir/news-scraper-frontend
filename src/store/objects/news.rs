use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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