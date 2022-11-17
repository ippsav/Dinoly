use std::path::PathBuf;

use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub address: String,
    pub port: u16,
}

impl Application {
    pub fn address(&self) -> String {
        format!("{}:{}", &self.address, self.port)
    }
}

#[derive(Debug, Deserialize)]
pub struct GlobalConfig {
    pub application: Application,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Environment {
    Test,
    Development,
    Production,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl From<String> for Environment {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "test" => Self::Test,
            "production" => Self::Production,
            _ => Self::Development,
        }
    }
}

impl GlobalConfig {
    pub fn build(env: Option<String>, path: PathBuf) -> Result<GlobalConfig, ConfigError> {
        let env = if let Some(value) = env {
            value.into()
        } else {
            Environment::default()
        };
        let file_name = match env {
            Environment::Test => "config.test.yaml",
            Environment::Development => "config.dev.yaml",
            Environment::Production => "config.prod.yaml",
        };
        let path = path.join(file_name);
        let source = path.to_str().expect("could not get path to config file");

        Config::builder()
            .add_source(File::new(source, FileFormat::Yaml))
            .build()?
            .try_deserialize::<GlobalConfig>()
    }
}
