use super::objects::scraper_info::ScraperInfo;
use crate::{helpers::request::request, Error};
use reqwest::Method;

pub struct AdminStore;

impl AdminStore {
    pub async fn get_scraper_info() -> Result<Vec<ScraperInfo>, Error> {
        request(Method::GET, "/admin/scrapers", None::<()>).await
    }
}
