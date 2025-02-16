mod routes;

pub async fn create_app() -> axum::Router {
    axum::Router::new()
        .nest("/users", routes::users::router())
        .nest("/chats", routes::chats::router())
}
