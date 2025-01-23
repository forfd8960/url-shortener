use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing::info;

use crate::{errors::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/shorten", post(shorten_url))
        .route("/redirect/{url_id}", get(redirect))
        .with_state(state);

    Ok(api_router)
}

async fn index() -> Result<impl IntoResponse, AppError> {
    Ok("Hello, World!")
}

async fn shorten_url(
    State(state): State<AppState>,
    Json(req): Json<ShortenRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("shorten url: {}", req.url);
    Ok("Hello, World!")
}

async fn redirect(
    Path(url_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("redirect url_id: {}", url_id);
    Ok("Hello, World!")
}
