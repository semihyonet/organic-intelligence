mod routes;

pub async fn create_app() -> axum::Router {
    let app = axum::Router::new().nest("/users", routes::users::router());
    app
}
