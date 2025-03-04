use std::path::Path;
use serde::Deserialize;
use crate::prelude::*;


#[derive(Clone, Deserialize)]
pub struct Secrets {
    pub postgres_connection_string: String,
}
impl Secrets {
    pub fn new<P: AsRef<Path>>(secret_path: P) -> Self {
        Self {
            postgres_connection_string: Self::read_secret_file(&secret_path).unwrap(),
        }
    }

    fn read_secret_file(path: &Path) -> Result<String> {
        if !path.exists() {
            return Err(Error::SecretNotFound(path.display().to_string()));
        }
        let secret_str = std::fs::read_to_string(path).unwrap();
        Ok(secret_str.trim().to_string())
    }

    pub fn postgres_connection_string(&self) -> &str {
        &self.postgres_connection_string
    }
}

#[derive(Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}
impl AppConfig {
    pub fn new(config_path: &str) -> Self {
        toml::from_str::<Self>(config_path).unwrap()
    }
}

#[derive(Clone)]
pub struct Config {
    secrets: Secrets,
    app: AppConfig,
}
impl Config {
    pub fn new(config_path: &str, secret_path: &str) -> Self {
        Self {
            app: AppConfig::new(config_path),
            secrets: Secrets::new(secret_path),
        }
    }
    pub fn config(&self) -> &AppConfig {
        &self.app
    }
    pub fn secret(&self) -> &Secrets {
        &self.secrets
    }
}