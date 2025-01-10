use crate::types::types::RequestError;
use crate::types::user::{User, UserFunc};
use axum::extract::{Path, State};
use axum::http::StatusCode;
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
    },
    Error(RequestError),
}

pub async fn get_info(
    State(session): State<Arc<Session>>,
    Path(user_id): Path<Uuid>,
) -> (StatusCode, Json<ReturnType>) {
    let user = User::from_user_id(user_id).fill_info(&session).await;
    if let Ok(user) = user{
        (StatusCode::OK, Json(ReturnType::ReturnData {
            createdat: user.createdat.unwrap_or(NaiveDate::MIN).format("%Y-%m-%d").to_string(),
            user_id: user.user_id,
            username: user.username,
            displayname: user.displayname,
            badges: user.badges,
            status: user.status,
            bannercolor: user.bannercolor,
            backgroundcolor: user.backgroundcolor,
        }))
    }else{
        (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Incorrect userId"))))
    }

}