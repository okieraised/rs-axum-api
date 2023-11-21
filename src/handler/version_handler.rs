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

pub async fn get_version() -> (StatusCode, Json<GenericResponse<'static>>)  {
    let json_response = GenericResponse {
        status: "",
        status_code: STATUS_NO_ERROR,
        message: "1.0.0",
        data: HashMap::new(),
    };

    (StatusCode::OK, Json(json_response))
}