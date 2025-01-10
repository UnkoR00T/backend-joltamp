use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use scylla::Session;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::security::passwords::verify_password;
use crate::types::types::{RequestError};
use crate::types::user::{User, UserFunc};

#[derive(Deserialize)]
pub struct RequestUser {
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnType {
    ReturnUser{
        user_id: Uuid,
        jwt: Uuid,
    },
    Error(RequestError),
}

pub async fn login(
    State(session): State<Arc<Session>>,
    Json(mut payload): Json<RequestUser>,
) -> (StatusCode, Json<ReturnType>) {
    
    let user: User = User::from_user_email(payload.email).fill_info(&session).await.unwrap();
    
    // Check if password was successfully fetched
    if let Some(password) = user.password {
        
        // Verify password
        if verify_password(&payload.password, &password).is_ok() {
            return (StatusCode::OK, Json(ReturnType::ReturnUser{ jwt: user.jwt.unwrap_or(Uuid::nil()), user_id: user.user_id.unwrap_or(Uuid::nil()), }));
        } else {
            return (StatusCode::UNAUTHORIZED, Json(ReturnType::Error(RequestError::from("Invalid password"))));
        };
    }

    (StatusCode::UNAUTHORIZED, Json(ReturnType::Error(RequestError::from("Ok"))))
}