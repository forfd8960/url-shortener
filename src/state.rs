use std::{collections::HashMap, ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

impl AppState {
    pub fn new() -> Self {
        let inner = Arc::new(AppStateInner {
            cache: HashMap::new(),
        });

        Self { inner }
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub cache: HashMap<String, String>,
}
