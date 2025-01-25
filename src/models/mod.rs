use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod urls;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, PartialEq)]
pub struct ShortURL {
    pub id: i64,
    pub origin_url: String,
    pub url_uid: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CreateShortURL {
    pub origin_url: String,
    pub url_uid: String,
}
