use awc::error;
use serde::Serialize;
use std::{error::Error, fmt::Display};
use actix_web::error::Error as ActixWebError;
use actix_web::error::ResponseError;
use actix_web::http::StatusCode;

#[derive(Debug, Serialize)]
pub enum MyError {
    TeraError(String),
    ActixError(String),
    NotFound(String)
}

pub struct MyErrorResponse {
    error_message: String
}

impl Error for MyError {}


impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl MyError {
    pub fn error_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);

                "Internal Server Error".into()
            },
            MyError::NotFound(msg) => {
                println!("Not Found error occured: {:?}", msg);

               msg.into()
            },
            MyError::TeraError(msg) => {
                println!("Error in rendering template: {:?}", msg);

                msg.into()
            }
        }
    }
}

impl From<ActixWebError> for MyError {
    fn from(error: ActixWebError) -> Self {
        MyError::ActixError(error.to_string())
    }
}

// 将 MyError 映射为 actix-web 的错误类型
// 为 MyError 实现 ResponseError trait，并 覆写 status_code 方法
impl ResponseError for MyError {
    fn status_code(&self) -> error::StatusCode {
        match self {
            MyError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            MyError::NotFound(_msg) => {
                StatusCode::NOT_FOUND
            },
            MyError::TeraError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
        
    }
}