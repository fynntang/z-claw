use std::path::PathBuf;

/// Platform abstraction for filesystem, shell, and environment access.
pub trait Platform: Send + Sync {
    fn data_dir(&self) -> PathBuf;
    fn config_dir(&self) -> PathBuf;
    fn home_dir(&self) -> PathBuf;
    fn os_name(&self) -> &str;
}

/// Real platform implementation using the `dirs` crate.
pub struct NativePlatform;

impl Platform for NativePlatform {
    fn data_dir(&self) -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("z-claw")
    }

    fn config_dir(&self) -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("z-claw")
    }

    fn home_dir(&self) -> PathBuf {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
    }

    fn os_name(&self) -> &str {
        std::env::consts::OS
    }
}
