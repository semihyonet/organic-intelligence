use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

pub async fn create_db_connection(url: String) -> Result<DatabaseConnection, DbErr> {
    let mut db = ConnectOptions::new(url);
    db.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    let connection = Database::connect(db).await?;

    Ok(connection)
}
