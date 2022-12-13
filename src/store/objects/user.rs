use std::f32::INFINITY;

use serde::{Deserialize, Serialize};

use crate::helpers::storage::get_user;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub id: String,
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

impl ToString for Plans{
    fn to_string(&self) -> String {
        match self{
            Self::Free => "Free",
            Self::Basic => "Basic",
            Self::Premium => "Premium",
            Self::Staff => "Staff"
        }.to_owned()
    }
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

impl Default for Plans {
    fn default() -> Self {
        Self::Free
    }
}

impl Default for User {
    fn default() -> Self {
        match get_user() {
            Some(user) => user,
            None => Self::empty()
        }
    }
}

impl User{
    pub fn empty() -> Self{
        Self {
            id: String::default(),
            username: String::default(),
            email: String::default(),
            created_at: String::default(),
            plan: Plans::default(),
            api_key: String::default(),
            credits_remaining: 0,
            credits_used: 0,
        }
    }
}
