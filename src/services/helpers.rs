use std::fmt::Debug;

use once_cell::sync::Lazy;
use reqwest::{Method, Client, header::HeaderValue};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, services::storage::get_key, error::ErrorJson};
use dotenv::var;

pub static API_ROOT: Lazy<String> = Lazy::new(|| {get_api_root()});

fn get_api_root() -> String{
    var("API").expect("API root is not set")
}

pub async fn request<B, T>(method: Method, route: &str, body: Option<B>) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + Debug,
    B: Serialize + Debug
{
    let url = format!("{}{}", API_ROOT.as_str(), route);

    let mut builder = Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    
    if let Some(key) = get_key(){
        builder = builder.header(
            "x-api-key",
            HeaderValue::from_str(&key).unwrap()
        );
    }
    
    if let Some(body) = body{
        builder = builder.json(&body);
    }

    let response = builder.send().await;
    
    match response{
        Ok(response) => {
            if response.status().is_success(){
                serde_json::from_str(
                    &response
                        .text()
                        .await
                        .map_err(|_| Error::RequestError)?
                    ).map_err(|_| Error::SerdeError)
            }else{
                let error = match response.json::<ErrorJson>().await{
                    Ok(error) => error,
                    Err(_) => return Err(Error::InternalServerError)
                };
                Err(error.into())
            }
        },
        Err(response) => {
            log::error!("Error fetching: {}", route);
            log::error!("Response code: {:#?}", response.status());
            log::error!("Response: {:#?}", response);
            Err(Error::RequestError)
        }
    }
}