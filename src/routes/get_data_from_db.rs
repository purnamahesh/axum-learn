use axum::{
    Extension, Json, response::{IntoResponse, Response}
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, prelude::FromRow};

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
struct Tasks {
    id: i32,
    priority: Option<String>,
    title: String,
    completed_at: Option<DateTime<Utc>>,
    description: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
    user_id: Option<i32>,
    is_default: bool
}

pub async fn get_data_from_db(Extension(conn): Extension<Pool<Postgres>>) -> Response {
    let result = sqlx::query_as::<_, Tasks>("select * from tasks").fetch_all(&conn).await;

    match result {
        Ok(rows) => {
            return (
                StatusCode::OK,
                Json(rows.to_vec())
            ).into_response()
        },
        Err(err) => return (
            StatusCode::INTERNAL_SERVER_ERROR,
            err.to_string()
        ).into_response()
    }
}
