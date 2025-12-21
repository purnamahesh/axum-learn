use axum::{
    Extension, Router, http::Method, routing::{get, post}
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body::{mirror_body_json, mirror_body_string};
use mirror_user_agent::mirror_user_agent;
use path_handler::path_handler;
use query_params::query_params;
use middleware_custom_header::middleware_custom_header;

mod hello_world;
mod middleware_message;
mod mirror_body;
mod mirror_user_agent;
mod path_handler;
mod query_params;
mod middleware_custom_header;

#[derive(Clone, Deserialize, Serialize)]
pub struct SharedData {
    pub msg: String,
}

pub fn create_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let shared_data = SharedData { msg: "Hello from Shread Data".to_string() };

    Router::new()
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/things/{id}", get(path_handler))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/middleware_message", get(middleware_message))
        .route("/middleware_custom_header", get(middleware_custom_header))
        .layer(cors)
        .layer(Extension(shared_data))
        
}
