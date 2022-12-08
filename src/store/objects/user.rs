use std::f32::INFINITY;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct User {
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

impl Default for Plans{
    fn default() -> Self {
        Self::Free
    }
}