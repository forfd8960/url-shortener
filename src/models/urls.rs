use crate::errors::AppError;

use super::ShortURL;
use sqlx::PgPool;

pub async fn create_short_url(pool: &PgPool, short_url: ShortURL) -> Result<ShortURL, AppError> {
    let res = sqlx::query_as(
        r#"
        INSERT INTO short_urls (origin_url, url_uid)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(short_url.origin_url)
    .bind(short_url.url_uid)
    .fetch_one(pool)
    .await?;

    Ok(res)
}

pub async fn get_origin_url_by_uid(
    pool: &PgPool,
    url_uid: &str,
) -> Result<Option<ShortURL>, AppError> {
    let res = sqlx::query_as(
        r#"
        SELECT id,origin_url,url_uid,created_at FROM short_urls
        WHERE url_uid = $1
        "#,
    )
    .bind(url_uid)
    .fetch_optional(pool)
    .await?;

    Ok(res)
}
