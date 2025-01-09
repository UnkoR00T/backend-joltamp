use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use scylla::Session;
use serde::{Deserialize, Serialize};
use serde::de::StdError;
use uuid::Uuid;
use crate::security::passwords::hash_password;
use crate::types::types::{RequestError, ReturnUser};

#[derive(Deserialize)]
pub struct RequestUser {
    email: String,
    password: String,
    username: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnType {
    ReturnUser(ReturnUser),
    Error(RequestError),
}

pub async fn register(
    State(session): State<Arc<Session>>,
    Json(mut payload): Json<RequestUser>,
) -> (StatusCode, Json<ReturnType>) {
    if payload.email.is_empty() || payload.password.is_empty() || payload.username.is_empty(){
        return (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Not every field satisfied"))));
    }
    if !payload.email.contains('@'){
        return (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Invalid e-mail address"))));
    }
    if payload.password.len() < 3 || payload.username.len() < 3{
        return (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Password or Username is too short (<4)"))));
    }

    if let Ok(used) = check_username_free(&session, &payload.username, &payload.email).await {
        if used {
            return (StatusCode::BAD_REQUEST, Json(ReturnType::Error(RequestError::from("Username/Email already used"))));
        }
    }else{
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(ReturnType::Error(RequestError::from("register#0x01 Internal server error"))));
    }
    if let Ok(user) = insert_user(&session, &mut payload).await{
        (StatusCode::CREATED, Json(ReturnType::ReturnUser(ReturnUser{
            jwt: user.0.to_string(),
            user_id: user.1.to_string(),
        })))
    }else{
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ReturnType::Error(RequestError::from("register#0x02 Internal server error"))))
    }
}


/// Checks if username is already used in database
/// 
/// Returns Ok(true) if username is already used
/// 
/// Returns Ok(false if username is free to use
async fn check_username_free(session: &Arc<Session>, username: &String, email: &String) -> Result<bool, Box<dyn StdError>> {
    let result = session.query_unpaged("SELECT user_id FROM joltamp.users WHERE username = ? ALLOW FILTERING", (username, )).await?.into_rows_result()?;
    let result2 = session.query_unpaged("SELECT user_id FROM joltamp.users WHERE email = ? ALLOW FILTERING", (email, )).await?.into_rows_result()?;

    Ok(result.rows_num() != 0 || result2.rows_num() != 0)
}

/// Pushes user to database
/// and returns generated jwt and user_id from
/// function to return it to end user
async fn insert_user(session: &Arc<Session>, payload: &mut RequestUser) -> Result<(Uuid, Uuid), Box<dyn StdError>> {
    let gen_jwt = Uuid::new_v4();
    let gen_user_id = Uuid::new_v4();
    hash_password(&mut payload.password).expect("Hashing error, disabling for safety.");
    session.query_unpaged("INSERT INTO joltamp.users (createdat, user_id, username, displayname, email, password, isadmin, jwt, status) VALUES (todate(now()), ?, ?, ?, ?, ?, false, ?, 0)",
                                     (gen_user_id, &payload.username, &payload.username, &payload.email, &payload.password, gen_jwt)
    ).await?;
    Ok((gen_jwt, gen_user_id))
}