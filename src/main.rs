mod routes;
mod security;
mod types;
mod functions;

use std::error::Error;
use std::sync::Arc;
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use scylla::{ExecutionProfile, Session, SessionBuilder};
use scylla::statement::Consistency;
use crate::routes::users::getinfo::get_info;
use crate::routes::users::register::register;
use crate::routes::users::login::login;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // SETUP SCYLLA DB
    let uri = std::env::var("SCYLLA_URI")
        .unwrap_or_else(|_| "172.17.0.2:9042".to_string());

    let handle = ExecutionProfile::builder()
        .consistency(Consistency::One)
        .build()
        .into_handle();

    let session: Session = SessionBuilder::new()
        .known_node(uri)
        .default_execution_profile_handle(handle)
        .build()
        .await?;

    let session = Arc::new(session);
    // SETUP AXUM

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/api/v0/", get(|| async {(StatusCode::OK, "All services running!")}))
        .route("/api/v0/users/register", post(register))
        .route("/api/v0/users/login", post(login))
        .route("/api/v0/users/getInfo/{id}", get(get_info))
        .with_state(session);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
