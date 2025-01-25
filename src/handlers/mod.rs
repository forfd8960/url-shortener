use nanoid::nanoid;
use url::Url;

use crate::{
    errors::AppError,
    models::{urls, CreateShortURL},
    state::AppState,
};

#[derive(Debug)]
pub struct Handler {
    pub state: AppState,
}

impl Handler {
    pub fn new(state: AppState) -> Self {
        Self { state }
    }

    pub async fn index(&self) -> Result<&str, AppError> {
        Ok("Hello, World!")
    }

    pub async fn gen_short_url(&self, origin_url: &str) -> Result<String, AppError> {
        // 1. validate the origin_url is a valid URL
        // 2. generate a unique url_uid
        // 3. save the origin_url and url_uid to the database
        // 4. return the short_url.

        let url = Url::parse(origin_url)?;
        match url.scheme() {
            "http" | "https" => {}
            _ => return Err(AppError::NotHTTPProtocol("Not HTTP protocol".to_string())),
        }

        let url_uid = nanoid!(20);
        let req = urls::create_short_url(
            &self.state.pool,
            CreateShortURL {
                origin_url: url.to_string(),
                url_uid: url_uid.clone(),
            },
        )
        .await?;

        Ok(format!("{}/r/{}", self.state.host, req.url_uid))
    }

    pub async fn get_origin_url(&self, url_id: &str) -> Result<String, AppError> {
        // 1. get the origin_url by url_id
        // 2. return the origin_url

        let res = urls::get_origin_url_by_uid(&self.state.pool, url_id).await?;

        match res {
            Some(url) => Ok(url.origin_url),
            None => Err(AppError::NotFound(url_id.to_string())),
        }
    }
}
