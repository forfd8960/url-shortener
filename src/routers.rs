use axum::{response::IntoResponse, routing::get, Router};

use crate::errors::AppError;

pub async fn get_router() -> Result<Router, AppError> {
    let api_router = Router::new().route("/index", get(index));

    Ok(api_router)
}

pub async fn index() -> impl IntoResponse {
    "Hello, World!"
}
