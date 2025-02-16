use axum::http;

pub async fn user_details() -> http::Response<String> {
    http::Response::new("User details".to_string())
}
