use serde::{Deserialize, Serialize};

use crate::{
    services::{storage::get_user, AuthStore},
    Error,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub plan: Plans,
    pub api_key: String,
    pub credits_used: i32,
    pub credits_remaining: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Plans {
    Free,
    Basic,
    Premium,
    Staff
}

impl Plans {
    pub fn get_max_calls(&self) -> i32 {
        match self {
            Self::Free => 100,
            Self::Basic => 20_000,
            Self::Premium => 50_000,
            Self::Staff => 100_000_00,
        }
    }
}

impl UserInfo {
    pub async fn login(username: &str, password: &str) -> Result<Self, Error> {
        AuthStore::login(username, password).await
    }
    pub async fn register(email: &str, username: &str, password: &str) -> Result<Self, Error> {
        AuthStore::register(email, username, password).await
    }
    pub fn empty() -> Self {
        UserInfo {
            username: "".to_owned(),
            api_key: "".to_owned(),
            created_at: "".to_owned(),
            email: "".to_owned(),
            plan: Plans::Free,
            credits_remaining: 0,
            credits_used: 0,
        }
    }
}

impl Default for UserInfo {
    fn default() -> Self {
        match get_user() {
            Some(user) => user,
            None => Self::empty(),
        }
    }
}
