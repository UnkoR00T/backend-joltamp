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
    ReturnData(Option<bool>),
    Error(RequestError),
}

/// This function checks if a user with the given `user_id` is an admin.
///
/// # Parameters
///
/// * `State(session)`: An instance of `Arc<Session>` representing the database session.
/// * `Path(user_id)`: A `Uuid` representing the user's unique identifier.
///
/// # Return Value
///
/// Returns a tuple containing a `StatusCode` and a `Json` object of type `ReturnType`.
///
/// * `StatusCode::OK`: If the user is successfully fetched from the database.
///   The `Json` object contains `ReturnType::ReturnData(Some(true))` if the user is an admin,
///   or `ReturnType::ReturnData(Some(false))` if the user is not an admin.
///
/// * `StatusCode::BAD_REQUEST`: If the user with the given `user_id` does not exist in the database.
///   The `Json` object contains `ReturnType::Error(RequestError::from("Incorrect userId"))`.
pub async fn is_admin(
    State(session): State<Arc<Session>>,
    Path(user_id): Path<Uuid>,
) -> (StatusCode, Json<ReturnType>) {
    let user = User::from_user_id(user_id).fill_info(&session).await;
    // Check if the user is fetched from db
    if let Ok(user) = user{
        // Returns data to user
        (StatusCode::OK, Json(ReturnType::ReturnData(user.isadmin)))
    }else{
        // Bad request error for non existing user
        (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Incorrect userId"))))
    }

}