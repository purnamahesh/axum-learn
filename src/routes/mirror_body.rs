use axum::Json;
use serde::{Deserialize, Serialize};

pub async fn mirror_body_string(body: String) -> String {
    body
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJSON {
    message: String,
}

#[derive(Debug, Serialize)]
pub struct MirrorJSONResponse {
    message: String,
    status_code: u8,
}

pub async fn mirror_body_json(Json(body): Json<MirrorJSON>) -> Json<MirrorJSONResponse> {
    Json(MirrorJSONResponse {
        message: body.message,
        status_code: 200,
    })
}
