use super::views;
use axum::{
    routing::{get, post, delete},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(views::list_chats))
        .route("/create", post(views::create_chat))
        .route("/:id_ref", get(views::chat_details))
        .route("/:id_ref", delete(views::destroy_chat))
}
