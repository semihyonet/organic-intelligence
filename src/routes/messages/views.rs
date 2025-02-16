use axum::extract::Path;
use axum::http;

pub async fn list_messages() -> http::Response<String> {
    http::Response::new("List of messages".to_string())
}

pub async fn create_message() -> http::Response<String> {
    http::Response::new("Create a message".to_string())
}

pub async fn message_details(Path(id_ref): Path<String>) -> http::Response<String> {
    http::Response::new(format!("Show a message details with id_ref: {}", &id_ref))
}

pub async fn destroy_message() -> http::Response<String> {
    http::Response::new("Destroy a message".to_string())
}
