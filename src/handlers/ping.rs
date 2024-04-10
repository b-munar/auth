pub async fn ping() -> String {
    return "ping".to_string();
}


#[cfg(test)]
mod tests {
    // use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},routing::get , Router
    };

    use tower:: ServiceExt;
    use dotenv::dotenv;

    // use utils::dbconn::{AppState,conn };

    use crate::{handlers::ping::ping, utils::dbconn::{conn, AppState}};



    #[tokio::test]
    async fn test_ping() {
        dotenv().ok();

        let state = AppState { conn: conn().await };

        let app = Router::new()
            .route("/ping",get(ping)).with_state(state);


        let response = app
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}