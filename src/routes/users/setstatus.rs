use std::sync::Arc;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use scylla::Session;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::types::{RequestError};
use crate::types::user::{User, UserFunc};

#[derive(Deserialize)]
pub struct RequestUser {
    status: u8,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnType {
    Ok,
    Error(RequestError),
}
pub async fn set_status(
    State(session): State<Arc<Session>>,
    headers: HeaderMap,
    Json(payload): Json<RequestUser>,
) -> (StatusCode, Json<ReturnType>) {
    
    if let Some(auth ) = headers.get("Authorization") {
        // Fetch user from db based on provided email
        let user = User::from_user_jwt(Uuid::parse_str(auth.to_str().unwrap_or("")).unwrap_or(Uuid::nil())).fill_info(&session).await;
        if let Ok(user) = user{
            let res = user.update(&session, "status", payload.status.to_string()).await;
            if let Err(err) = &res {
                (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from(err.to_string()))))
            }else{
                (StatusCode::OK, Json(ReturnType::Ok))
            }
        }else{
            (StatusCode::UNAUTHORIZED, Json(ReturnType::Error(RequestError::from("User JWT not found"))))
        }
    }else{
        (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Invalid JWT"))))
    }

}