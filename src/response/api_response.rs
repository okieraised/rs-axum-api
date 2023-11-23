use std::any::Any;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{Value, Map, Number};
use once_cell::sync::Lazy;

pub const STATUS_NO_ERROR: i8 = 0;
pub const STATUS_BAD_REQUEST: i8 = 1;
pub const STATUS_REQUEST_TIMEOUT_ERROR: i8 = 2;
pub const STATUS_INTERNAL_SERVER_ERROR: i8 = 3;


pub const STATUS_NO_ERROR_STR: &str = "OK";
pub const STATUS_BAD_REQUEST_STR: &str = "Bad Request";
pub const STATUS_REQUEST_TIMEOUT_ERROR_STR: &str = "Request Timeout";
pub const STATUS_INTERNAL_SERVER_ERROR_STR: &str = "Internal Server Error";


pub static STATUS_MAPPER: Lazy<HashMap<i8, &str>> = Lazy::new(|| HashMap::from(
    [
        (STATUS_NO_ERROR, STATUS_NO_ERROR_STR),
        (STATUS_BAD_REQUEST, STATUS_BAD_REQUEST_STR),
        (STATUS_REQUEST_TIMEOUT_ERROR, STATUS_REQUEST_TIMEOUT_ERROR_STR),
        (STATUS_INTERNAL_SERVER_ERROR, STATUS_INTERNAL_SERVER_ERROR_STR),
    ],
));

/// Define the common response for all api call
#[derive(Debug, Serialize)]
pub struct GenericResponse<'a> {
    /// The status string for the response
    pub status: &'a str,
    /// The status code for the response
    pub status_code: i8,
    /// the message string for the response
    pub message: &'a str,
    /// the optional data map for the response
    pub data: HashMap<&'a str, Value>,
}
