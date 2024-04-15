use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;

use sha256::digest;

use crate::{utils, AppState};

use serde::{
    Deserialize,
    Serialize
};

use entity::user;

use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    QueryFilter,Set
};


use serde_json;

use utils::create_token::create_token;

pub async fn user_register(
    State(state): State<AppState>,
    Json(payload): Json<UserDeserialize>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let email_is_registered = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email.clone()))
        .one(&state.conn)
        .await
        .map_err(|e| {
            let error_database = serde_json::json!({
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
        })
        .unwrap()
        .is_some();

    if email_is_registered {
        let error_email_is_registered = serde_json::json!({
            "message": "this email is registered",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_email_is_registered)));
    }

    let hashed_password = digest(payload.password);


    let mut user_model = user::ActiveModel {
        email: Set(payload.email),
        password: Set(hashed_password),
        id: Set(payload.id),
        role: Set(user::Role::NotRole)
    };

    if payload.role == 1 {
        user_model.role = Set(user::Role::Sportmen)
    }

    else if payload.role ==2 {
        user_model.role = Set(user::Role::Partner)
    }

    else {
        let error_email_is_registered = serde_json::json!({
            "message": "this role no exist",
        });
        return Err((StatusCode::BAD_REQUEST, Json(error_email_is_registered)));
    }

    let user_model_insert: user::Model = user_model
        .insert(&state.conn)
        .await
        .map_err(|e| {
            let error_database = serde_json::json!({
                "message": format!("Database error: {}", e),
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_database))
        })
        .unwrap();


        let role = user_model_insert.role as i32;

        let token = create_token(user_model_insert.id, role);

        let authenticate = serde_json::json!({"auth": {"token": token, "email": user_model_insert.email, "role": role}});

    Ok((StatusCode::CREATED, Json(authenticate)))
}

#[derive(Deserialize)]
pub struct UserDeserialize {
    email: String,
    password: String,
    id: Uuid,
    role: i32
}

#[derive(Serialize, Deserialize)]
pub struct GetUser {
    id: Uuid,
}


#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode, Method},routing::post , Router
    };

    use tower:: ServiceExt;
    use dotenv::dotenv;

    use crate::{handlers::user_register::user_register, utils::dbconn::{conn, AppState}};



    #[tokio::test]
    async fn test_register() {
        dotenv().ok();

        let state = AppState { conn: conn().await };

        let app = Router::new()
            .route("/register", post(user_register))
            .with_state(state);

        let response = app
            .oneshot(Request::builder().method(Method::POST).uri("/register")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "email": "partnerr@email.com",
                    "password": "password",
                    "id": "28a3ad87-7d3c-47e3-9c42-858ca3ec5222",
                    "role": 2
                }"#,
            )).unwrap())
            .await
            .unwrap();



        assert_eq!(response.status(), StatusCode::ACCEPTED);
    }
}