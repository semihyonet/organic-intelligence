use super::views;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/details", get(views::user_details))
}
