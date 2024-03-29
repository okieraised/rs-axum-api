use std::collections::HashMap;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use log::info;
use serde_json::{Value, Map, Number};

use crate::response::api_response::*;

const API_VERSION: &str = "1.0.0";

pub async fn get_version() -> (StatusCode, Json<GenericResponse<'static>>)  {
    let json_response = GenericResponse {
        status: STATUS_MAPPER.get(&STATUS_NO_ERROR).unwrap_or(&STATUS_NO_ERROR_STR),
        status_code: STATUS_NO_ERROR,
        message: API_VERSION,
        data: HashMap::new(),
    };

    (StatusCode::OK, Json(json_response))
}