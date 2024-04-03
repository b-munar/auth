use axum::{
    extract::Json,
    http::StatusCode,
};

use uuid::Uuid;

use serde::{
    Deserialize,
    Serialize
};


use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json;


pub fn create_token(user_id: Uuid, role_id: i32) -> String{

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(3)).timestamp() as usize;

    let claims = Claims{
        sub: user_id.to_string(),
        role: role_id,
        exp,
        iat,
    };

    let secret = std::env::var("AUTH_SECRET")
    .unwrap();

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref())
    ).map_err(|e| {
        let error_jwt = serde_json::json!({
            "message": format!("jwt token creation error: {}", e),
        });
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_jwt))
    })
    .unwrap();

    return token

}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    role: i32
}