use axum::Router;

use sea_orm::{Database, DatabaseConnection};

use dotenv::dotenv;

use routes::auth_router;

mod handlers;
mod routes;
mod utils;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
    .with_target(false)
    .compact()
    .init();

    let state = AppState { conn: conn().await };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();

    axum::serve(listener, app(auth_router(), state)).await.unwrap();
}

pub async fn conn()->DatabaseConnection{
    let database_url = std::env::var("AUTH_DATABASE_URL_PATH").unwrap();
    
    return Database::connect(database_url)
        .await
        .expect("Database connection failed");
}

pub fn app(auth_router: Router<AppState>, state:AppState) -> Router {
    Router::new().nest("/auth", auth_router).with_state(state)
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new()
                .level(Level::INFO))
            .on_response(trace::DefaultOnResponse::new()
                .level(Level::INFO)),

    )
}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };

    use tower:: ServiceExt;
    use dotenv::dotenv;

    use routes::auth_router;

    #[tokio::test]
    async fn ping() {
        dotenv().ok();


        let database_url = std::env::var("AUTH_DATABASE_URL_PATH")
            .unwrap();
    
        let conn = Database::connect(database_url)
            .await
            .expect("Database connection failed");
    
        let state = AppState { conn };
        let app = app(auth_router(), state);

        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = app
            .oneshot(Request::builder().uri("/auth/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}