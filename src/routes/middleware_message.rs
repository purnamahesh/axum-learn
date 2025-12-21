use axum::{Extension, Json};

use crate::routes::SharedData;

pub async fn middleware_message(Extension(shared_data): Extension<SharedData>) 
    -> Json<SharedData>
{
    Json(shared_data)
}
