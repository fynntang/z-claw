pub mod config;
pub mod service;
pub mod types;

pub use config::{AppConfig, ValidationResult};
pub use service::DesktopBridge;
pub use types::{ChatInput, RuntimeStatus};
