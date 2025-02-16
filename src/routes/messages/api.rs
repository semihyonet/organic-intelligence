use super::views;
use axum::{
    routing::{get, post, delete},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/", get(views::list_messages))
        .route("/create", post(views::create_message))
        .route("/:id_ref", get(views::message_details))
        .route("/:id_ref", delete(views::destroy_message))
}
