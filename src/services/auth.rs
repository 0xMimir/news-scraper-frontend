use reqwest::Method;
use serde::Serialize;

use crate::{store::UserInfo, Error};

use super::helpers::request;

#[derive(Serialize, Debug)]
struct LoginForm{
    username: String,
    password: String
}

#[derive(Serialize, Debug)]
struct RegisterForm{
    email: String,
    username: String,
    password: String
}

pub struct AuthStore;

impl AuthStore{
    pub async fn login(
        username: &str, 
        password: &str
    ) -> Result<UserInfo, Error>{
        let form = LoginForm{
            username: username.to_owned(),
            password: password.to_owned()
        };
        request(
            Method::POST,
            "/auth/login",
            Some(form)
        ).await
    }
    
    pub async fn current() -> Result<UserInfo, Error>{
        request(
            Method::GET,
            "/auth/self",
            None::<()>
        ).await
    }
    
    pub async fn register(
        email: &str,
        username: &str,
        password: &str
    ) -> Result<UserInfo, Error>{
        let form = RegisterForm{
            email: email.to_owned(),
            username: username.to_owned(),
            password: password.to_owned()
        };
        request(
            Method::POST,
            "/auth/register",
            Some(form)
        ).await
    }
}