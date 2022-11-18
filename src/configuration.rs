use serde_aux::field_attributes::deserialize_number_from_string;
use std::path::PathBuf;

use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Application {
    pub address: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
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
pub enum Env {
    Test,
    Development,
    Production,
}

impl Default for Env {
    fn default() -> Self {
        Self::Development
    }
}

impl From<String> for Env {
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
            Env::default()
        };
        if env == Env::Production {
            Self::set_port_for_prod();
        }
        let file_name = match env {
            Env::Test => "config.test.yaml",
            Env::Development => "config.dev.yaml",
            Env::Production => "config.prod.yaml",
        };
        let path = path.join(file_name);
        let source = path.to_str().expect("could not get path to config file");

        Config::builder()
            .add_source(File::new(source, FileFormat::Yaml))
            .add_source(
                Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?
            .try_deserialize::<GlobalConfig>()
    }
    fn set_port_for_prod() {
        let port = std::env::var("PORT").unwrap();
        std::env::set_var("APP_APPLICATION__PORT", port);
    }
}
