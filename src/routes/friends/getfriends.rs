use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use scylla::Session;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::friend::Friend;
use crate::types::types::RequestError;
use crate::types::user::{User, UserFunc};

#[derive(Deserialize)]
pub struct RequestUser {

}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnType {
    ReturnFriends(HashMap<Uuid, Friend>),
    Error(RequestError),
}
pub async fn get_friends(
    State(session): State<Arc<Session>>,
    headers: HeaderMap,
) -> (StatusCode, Json<ReturnType>) {
    if let Some(auth ) = headers.get("Authorization") {
        let user: User = User::from_user_jwt(Uuid::parse_str(auth.to_str().unwrap()).unwrap_or(Uuid::nil())).fill_info(&session).await.unwrap().fetch_friends(&session).await.unwrap();

        if let Some(friends) = user.friends{
            return (StatusCode::OK, Json(ReturnType::ReturnFriends(friends)));
        }else{
            return (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Cannot fetch friends"))))
        }

    }

    (StatusCode::UNAUTHORIZED, Json(ReturnType::Error(RequestError::from("Ok"))))
}