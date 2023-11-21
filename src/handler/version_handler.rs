use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::response::api_response::*;

pub async fn get_version() -> (StatusCode, Json<GenericResponse>)  {
    let json_response = serde_json::json!(
        GenericResponse {
            status: String::from(""),
            status_code: STATUS_NO_ERROR,
            message: String::from("1.0.0"),
            // data: Default::default(),
        }
    );
    (StatusCode::OK, Json(GenericResponse {
        status: String::from(""),
        status_code: STATUS_NO_ERROR,
        message: String::from("1.0.0"),
        // data: Default::default(),
    }))
}