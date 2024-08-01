use actix_web::{error, http::StatusCode};
use std::fmt::{write, Display};

use sqlx::error::Error as SQLxError;


// todo 使用 derive_more 这个宏加载实现更多的 Trait，如 Display
#[derive(Debug)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}


pub struct MyErrorResponse {
    error_message: String
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::ActixError(msg) => {
                println!("Server error occured: {}", msg);
                msg.into()
            },
            MyError::DBError(msg) => {
                println!("Database error occured: {}", msg);
                msg.into()
            },
            MyError::NotFound(msg) => {
                println!("Not Found error occured: {}", msg);
                msg.into()
            },
            MyError::InvalidInput(msg) => {
                println!("Invalid Input error occured: {}", msg);
                msg.into()
            },
            
        }
    }
}

// 为 MyError 实现 actix_web 中的 error::ResponseError trait，可以自动将自定义 error 转换为 actix_web 中的 error 类型
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            },
            MyError::NotFound(_msg) => {
                StatusCode::NOT_FOUND
            },
            MyError::InvalidInput(_msg) => {
                StatusCode::BAD_REQUEST
            }
        }
    }
   
}

impl Display for MyError {
    // 实现 fmt 方法
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 使用 write! 这个宏向指定目标写入数据
        write!(f, "{}", self)
    }
}

// 为 MyError 实现 From trait，以实现 error:Error 到 MyError 的转换
// 即 actix_web 的错误类型转换为自定义错误类型
impl From<error::Error> for MyError {
    fn from(err: error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(error: SQLxError) -> Self {
        MyError::DBError(error.to_string())
    }
}

