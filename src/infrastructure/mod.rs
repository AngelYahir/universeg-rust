pub mod config;
pub mod errors;
pub mod logger;
pub mod postgres;
pub mod security;
pub mod mongo {
    pub mod connection;
    pub mod models;
}

pub mod swagger {
    pub mod config;
}
