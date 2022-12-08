use std::f32::INFINITY;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::helpers::{error::Error, request::request, storage::get_user};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserStore {
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
    Staff,
}

impl Plans {
    pub fn get_max_calls(&self) -> f32 {
        match self {
            Self::Free => 100.0,
            Self::Basic => 20_000.0,
            Self::Premium => 50_000.0,
            Self::Staff => INFINITY,
        }
    }
}

#[derive(Serialize, Debug)]
struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

impl UserStore {
    pub async fn login(username: &str, password: &str) -> Result<Self, Error> {
        let form = LoginForm {
            username: username.to_owned(),
            password: password.to_owned(),
        };
        request(Method::POST, "/auth/login", Some(form)).await
    }
    pub async fn register(email: &str, username: &str, password: &str) -> Result<Self, Error> {
        let form = RegisterForm {
            email: email.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        };
        request(Method::POST, "/auth/register", Some(form)).await
    }
    pub fn empty() -> Self {
        Self {
            username: "".to_owned(),
            api_key: "".to_owned(),
            created_at: "".to_owned(),
            email: "".to_owned(),
            plan: Plans::Free,
            credits_remaining: 0,
            credits_used: 0,
        }
    }

    pub async fn current() -> Result<Self, Error> {
        request(Method::GET, "/auth/self", None::<()>).await
    }
}

impl Default for UserStore {
    fn default() -> Self {
        match get_user() {
            Some(user) => user,
            None => Self::empty(),
        }
    }
}
