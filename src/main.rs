mod handler;
mod util;
mod authentication;
mod database;
mod middleware;
mod routes;
mod response;
mod constants;
mod logging;

use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::handler::*;
use log::{debug, error, info};
use std::env;
use ecs_logger::extra_fields;



#[tokio::main]
async fn main() {

    env::set_var("RUST_LOG", "info");
    // initialize tracing
    // tracing_subscriber::fmt::init();

    ecs_logger::init();

    // extra_fields::set_extra_fields(MyExtraFields {
    //     my_field: "my_value".to_string(),
    // }).unwrap();

    // build our application with a route
    let app = Router::new()
        .route("/", get(version_handler::get_version));
        // // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
