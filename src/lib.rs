use crate::database::create_db_connection;
use crate::settings::get_configuration;
mod routes;
mod database;

mod settings;

pub async fn create_app() -> axum::Router {
    let x= get_configuration().await.expect("TODO: panic message");

    let db = create_db_connection(x.database_url).await.expect("db connection fails");

    assert!(db.ping().await.is_ok());
    db.clone().close().await.expect("TODO: panic message");
    assert!(matches!(db.ping().await, Err(_DbErr)));


    axum::Router::new()
        .nest("/users", routes::users::router())
        .nest("/chats", routes::chats::router())
        .nest("/messages", routes::messages::router())
}
