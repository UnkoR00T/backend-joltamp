use crate::types::types::RequestError;
use crate::types::user::{User, UserFunc};
use axum::extract::{State};
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

/// Retrieves the self information of a user based on the provided JWT in the headers.
///
/// # Parameters
/// - `State(session)`: An `Arc` wrapped `Session` object used to interact with the database.
/// - `headers`: A `HeaderMap` containing the HTTP headers, which should include the "Authorization" header with the user's JWT.
///
/// # Returns
/// A tuple containing:
/// - `StatusCode`: The HTTP status code indicating the result of the operation.
/// - `Json<ReturnType>`: A JSON response containing either the user's information or an error message.
pub async fn get_self_info(
    State(session): State<Arc<Session>>,
    headers: HeaderMap,
) -> (StatusCode, Json<ReturnType>) {
    // Check if Authorization header is present
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
        // Bad request error for missing Authorization header
        (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Invalid JWT"))))
    }
}