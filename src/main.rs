use axum::Router;

use dotenv::dotenv;

use routes::auth_router;

use utils::dbconn::{AppState,conn };

mod handlers;
mod routes;
mod utils;

use tower_http::trace::{self, TraceLayer};
use tracing::Level;


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
