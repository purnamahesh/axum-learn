use axum::http::HeaderMap;

pub async fn mirror_user_agent(headers: HeaderMap) -> String {
    return if let Some(user_agent_hv) = headers.get("User-Agent") {
        match user_agent_hv.to_str() {
            Ok(ua) => ua.to_owned(),
            Err(err) => err.to_string(),
        }
    } else {
        "Header doesn't exist".to_owned()
    };
}
