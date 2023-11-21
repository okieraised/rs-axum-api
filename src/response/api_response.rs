use std::any::Any;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json::{Value, Map, Number};


pub const STATUS_NO_ERROR: i8 = 0;
pub const STATUS_BAD_REQUEST: i8 = 1;
pub const STATUS_REQUEST_TIMEOUT_ERROR: i8 = 2;
pub const STATUS_INTERNAL_SERVER_ERROR: i8 = 3;


pub static STATUS_MAPPER: HashMap<i8, &str> = HashMap::from(
    [
        (STATUS_NO_ERROR, "OK"),
        (STATUS_BAD_REQUEST, "Bad Request"),
        (STATUS_REQUEST_TIMEOUT_ERROR, "Request Timeout"),
        (STATUS_INTERNAL_SERVER_ERROR, "Internal Server Error"),
    ],
);

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
