use std::any::Any;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub const STATUS_NO_ERROR: i8 = 0;
pub const STATUS_BAD_REQUEST: i8 = 1;
pub const STATUS_REQUEST_TIMEOUT_ERROR: i8 = 2;
pub const STATUS_INTERNAL_SERVER_ERROR: i8 = 3;


/// Define the common response for all api call
///
///
///
///
#[derive(Debug, Serialize)]
pub struct GenericResponse {
    /// The status string for the response
    pub status: String,
    /// The status code for the response
    pub status_code: i8,
    /// the message string for the response
    pub message: String,
    // the optional data map for the response
    // pub data: Option<HashMap<String, Box<dyn Any>>>,
}
