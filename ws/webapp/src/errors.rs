use actix_web::{error, http::StatusCode, HttoResponse, Result};
use serde::Serialize;
use std::fmt;

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
