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
}
