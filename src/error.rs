use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct ErrorJson{
    pub message: String,
    pub error: Option<Value>,
    pub code: i32
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error{
    RequestError,
    SerdeError,
    InternalServerError,
    UserNotFound,
    NotFound,
    InvalidInput,
    Unauthorized
}

impl From<StatusCode> for Error{
    fn from(err: StatusCode) -> Self{
        match err.as_u16(){
            400 => Self::InvalidInput,
            404 => Self::NotFound,
            401 => Self::Unauthorized,
            _ => Self::InternalServerError
        }
    }
}

impl From<ErrorJson> for Error{
    fn from(error: ErrorJson) -> Self{
        match error.code{
            400 => Self::InvalidInput,
            401 => Self::Unauthorized,
            404 => {
                match error.message.as_str(){
                    "user-not-found" => Self::UserNotFound,
                    _ => Self::NotFound
                }
            },
            _ => Self::InternalServerError
        }
    }
}

impl ToString for Error{
    fn to_string(&self) -> String {
        match self{
            Error::RequestError => "Request Error",
            Error::SerdeError => "Parse Error",
            Error::InternalServerError => "Internal server error",
            Error::UserNotFound => "User not found",
            Error::NotFound => "Not found",
            Error::InvalidInput => "Invalid input",
            Error::Unauthorized => "Unathorized"
        }.to_owned()
    }
}