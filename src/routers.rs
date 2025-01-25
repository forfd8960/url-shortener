use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use tracing::info;

use crate::{errors::AppError, handlers::Handler, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api_router = Router::new()
        .route("/index", get(index))
        .route("/shorten", post(shorten_url))
        .route("/r/{url_id}", get(redirect))
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

    let handler = Handler::new(state);
    let short_url = handler.gen_short_url(&req.url).await?;
    info!("generated short URL: {}", short_url);

    Ok(short_url)
}

async fn redirect(
    Path(url_id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    info!("redirect url_id: {}", url_id);

    let origin_url = Handler::new(state).get_origin_url(&url_id).await?;
    Ok(Redirect::permanent(&origin_url).into_response())
}
