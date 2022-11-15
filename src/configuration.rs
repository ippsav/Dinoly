use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppSettings {
    pub address: String,
    pub port: u16,
}

impl AppSettings {
    pub fn address(&self) -> String {
        format!("{}:{}", &self.address, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub app_settings: AppSettings,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Test,
    Development,
    Production,
}

impl From<&str> for Environment {
    fn from(value: &str) -> Self {
        match value {
            "test" => Self::Test,
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

impl GlobalConfig {
    pub fn build(env: &str) -> Result<GlobalConfig, ConfigError> {
        let source_file = match env.into() {
            Environment::Test => "config/config.test.yaml",
            Environment::Development => "config/config.dev.yaml",
            Environment::Production => "config/config.prod.yaml",
        };

        Config::builder()
            .add_source(File::new(source_file, FileFormat::Yaml))
            .build()?
            .try_deserialize::<GlobalConfig>()
    }
}
