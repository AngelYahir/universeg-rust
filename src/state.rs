use crate::infra::{config::Config, db::DbPool};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: DbPool,
}

impl AppState {
    pub fn new(cfg: Config, db: DbPool) -> Self {
        Self {
            cfg: Arc::new(cfg),
            db,
        }
    }
}
