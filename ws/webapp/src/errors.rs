use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;
use std::fmt::Formatter;

// 允许出现未使用的代码，使得编译器不发生告警
#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub enum MyError {
    ActixError(String),
    NotFound(String),
    TeraError(String)
}
    
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String
}

impl std::error::Error for MyError {}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occured: {:?}", msg);
                "Interval server error".into()
            }
            MyError::TeraError(msg) => {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => {
                println!("Not Found error occured: {:?}", msg);
                msg.into()
            }
        }
    }
}



impl fmt::Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(msg: actix_web::error::Error) -> Self {
        MyError::ActixError(msg.to_string())
    }
}