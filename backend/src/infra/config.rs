use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use crate::prelude::*;


#[derive(Debug, Clone, Deserialize)]
pub struct Secrets {
    postgres_connection_string: String,
}
impl Secrets {
    pub fn new<P: AsRef<Path>>(secret_path: P) -> Self {
        let base_path = PathBuf::from(secret_path.as_ref());
        let pg_connection_string = Self::read_secret_file(&base_path.join("postgres-connection-string")).unwrap();
        Self {
            postgres_connection_string: pg_connection_string,
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
    pub fn new(config_str: &str) -> Self {
        toml::from_str::<Self>(config_str).unwrap()
    }
}

#[derive(Clone)]
pub struct Config {
    secrets: Secrets,
    app: AppConfig,
}
impl Config {
    pub fn new(config_path: &str, secret_path: &str) -> Self {
        let config_str = fs::read_to_string(&config_path).unwrap();
        Self {
            app: AppConfig::new(config_str.as_ref()),
            secrets: Secrets::new(secret_path),
        }
    }
    pub fn app(&self) -> &AppConfig {
        &self.app
    }
    pub fn secret(&self) -> &Secrets {
        &self.secrets
    }
}