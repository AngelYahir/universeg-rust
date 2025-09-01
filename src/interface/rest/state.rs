use crate::app::usecases::auth::{login::LoginHandler, register::RegisterHandler};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApiDeps {
    pub login_handler: Arc<dyn LoginHandler>,
    pub register_handler: Arc<dyn RegisterHandler>,
}
