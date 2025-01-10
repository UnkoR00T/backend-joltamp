use crate::types::types::RequestError;
use crate::types::user::{User, UserFunc};
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use scylla::Session;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RequestUser {

}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnType {
    ReturnData{
        createdat: String,
        user_id: Option<Uuid>,
        username: Option<String>,
        displayname: Option<String>,
        badges: Option<Vec<Uuid>>,
        status: Option<i8>,
        bannercolor: Option<String>,
        backgroundcolor: Option<String>,
        email: Option<String>,
    },
    Error(RequestError),
}

pub async fn get_self_info(
    State(session): State<Arc<Session>>,
    headers: HeaderMap,
) -> (StatusCode, Json<ReturnType>) {
    if let Some(auth) = headers.get("Authorization") {
        let user = User::from_user_jwt(Uuid::parse_str(auth.to_str().unwrap_or("")).unwrap_or(Uuid::nil())).fill_info(&session).await;
        // Check if the user is fetched from db
        if let Ok(user) = user{
            // Returns data to user
            (StatusCode::OK, Json(ReturnType::ReturnData {
                createdat: user.createdat.unwrap_or(NaiveDate::MIN).format("%Y-%m-%d").to_string(),
                user_id: user.user_id,
                username: user.username,
                displayname: user.displayname,
                badges: user.badges,
                status: user.status,
                bannercolor: user.bannercolor,
                backgroundcolor: user.backgroundcolor,
                email: user.email,
            }))
        }else{
            // Bad request error for non existing user
            (StatusCode::UNAUTHORIZED, Json(ReturnType::Error(RequestError::from("User JWT not found"))))
        }   
    }else {
        (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Invalid JWT"))))
    }
}