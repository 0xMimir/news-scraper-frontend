use serde::{Deserialize, Serialize};

use crate::{Error, services::{AuthStore, storage::get_user}};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserInfo{
    pub username: String,
    pub api_key: String,
    pub created_at: String,
    pub email: String,
    pub plan: String
}

impl UserInfo{
    pub async fn login(username: &str, password: &str) -> Result<Self, Error>{
        AuthStore::login(username, password).await
    }
    pub async fn register(
        email: &str,
        username: &str,
        password: &str
    ) -> Result<Self, Error>{
        AuthStore::register(email, username, password).await
    }
}

impl Default for UserInfo{
    fn default() -> Self {
        match get_user(){
            Some(user) => user,
            None => UserInfo{
                username: "".to_owned(),
                api_key: "".to_owned(),
                created_at: "".to_owned(),
                email: "".to_owned(),
                plan: "".to_owned()
            }
        }
    }
}