
use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

pub async fn conn()->DatabaseConnection{
    let database_url = std::env::var("AUTH_DATABASE_URL_PATH").unwrap();
    
    return Database::connect(database_url)
        .await
        .expect("Database connection failed");
}
