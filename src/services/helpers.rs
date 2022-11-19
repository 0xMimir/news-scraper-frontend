use std::fmt::Debug;

use dotenv_codegen::dotenv;
use reqwest::{Method, Client, header::HeaderValue};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, services::storage::get_key, error::ErrorJson};

pub const API_ROOT: &str = dotenv!("API");

pub async fn request<B, T>(method: Method, route: &str, body: Option<B>) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + Debug,
    B: Serialize + Debug
{
    let url = format!("{}{}", API_ROOT, route);

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