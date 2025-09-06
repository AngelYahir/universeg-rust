use std::sync::Arc;

#[derive(Clone)]
pub struct CoreState {
    pub jwt_service: Arc<dyn crate::app::ports::JwtService>,
    pub pool: crate::infrastructure::postgres::pool::DbPool,
    #[warn(dead_code)]
    pub mongo_db: crate::infrastructure::mongo::connection::MongoDb,
}
