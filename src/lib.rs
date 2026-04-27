pub mod backend;
pub mod config;
pub mod error;
pub mod frontend;
mod metrics;
pub mod models;
pub mod registry;
pub mod routes;

pub use config::AppConfig;
pub use error::AppError;
pub use registry::ModelRegistry;
