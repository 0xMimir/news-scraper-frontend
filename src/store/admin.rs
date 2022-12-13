use std::time::Duration;

use super::objects::scraper_info::ScraperInfo;
use crate::{helpers::request::request, Error};
use reqwest::Method;
use yew::{
    platform::{spawn_local, time::sleep},
    Callback,
};

pub struct AdminStore;

impl AdminStore {
    pub async fn get_scraper_info() -> Result<Vec<ScraperInfo>, Error> {
        request(Method::GET, "/admin/scrapers", None::<()>).await
    }

    pub fn update_scraper_info(callback: Callback<Vec<ScraperInfo>>) {
        spawn_local(async move {
            loop {
                if let Ok(result) = Self::get_scraper_info().await {
                    callback.emit(result);
                }
                sleep(Duration::from_secs(10)).await;
            }
        })
    }
}
