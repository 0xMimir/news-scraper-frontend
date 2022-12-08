use reqwest::Method;
use serde::Serialize;

use crate::helpers::{error::Error, request::request};

use super::objects::user::User;

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

pub struct UserStore;

impl UserStore {
    pub async fn login(username: &str, password: &str) -> Result<User, Error> {
        let form = LoginForm {
            username: username.to_owned(),
            password: password.to_owned(),
        };
        request(Method::POST, "/auth/login", Some(form)).await
    }
    pub async fn register(email: &str, username: &str, password: &str) -> Result<User, Error> {
        let form = RegisterForm {
            email: email.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        };
        request(Method::POST, "/auth/register", Some(form)).await
    }
    pub async fn current() -> Result<User, Error> {
        request(Method::GET, "/auth/self", None::<()>).await
    }
}
