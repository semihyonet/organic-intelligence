use axum::extract::Path;
use axum::http;

pub async fn list_chats() -> http::Response<String> {
    http::Response::new("List of chats".to_string())
}

pub async fn create_chat() -> http::Response<String> {
    http::Response::new("Create a chat".to_string())
}

pub async fn chat_details(Path(id_ref): Path<String>) -> http::Response<String> {
    http::Response::new(format!("Show a chat details with id_ref: {}", &id_ref))
}

pub async fn destroy_chat() -> http::Response<String> {
    http::Response::new("Destroy a chat".to_string())
}
