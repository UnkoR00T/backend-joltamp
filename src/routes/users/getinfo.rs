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

/// Retrieves user information from the database based on the provided user ID.
///
/// # Parameters
///
/// * `State(session)`: An instance of `Arc<Session>` representing the database session.
/// * `Path(user_id)`: A `Uuid` representing the user ID for which to retrieve information.
///
/// # Return
///
/// Returns a tuple containing a `StatusCode` and a `Json` object of type `ReturnType`.
///
/// * `StatusCode::OK`: If the user information is successfully retrieved.
/// * `ReturnType::ReturnData`: Contains the user information in the specified format.
/// * `StatusCode::BAD_REQUEST`: If the provided user ID is incorrect.
/// * `ReturnType::Error`: Contains an error message indicating the incorrect user ID.
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