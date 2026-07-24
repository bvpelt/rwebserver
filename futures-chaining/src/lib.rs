/// -----------------------------------------------------------------------
/// Application Configuration Central Hierarchy
/// -----------------------------------------------------------------------
pub struct AppConfig {
    pub version: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let version = std::env::var("VERSION").unwrap();
        Self { version }
    }
}
