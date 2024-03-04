use std::sync::OnceLock;
use typesense_codegen::apis::configuration::{ApiKey, Configuration};

mod collection;
mod documents;

static CONFIG: OnceLock<Configuration> = OnceLock::new();

#[cfg(not(target_arch = "wasm32"))]
fn init() -> Configuration {
    let _ = dotenvy::dotenv();

    let base_path = std::env::var("URL").expect("URL must be present in .env");
    let key = std::env::var("API_KEY").expect("API_KEY must be present in .env");

    Configuration {
        base_path,
        api_key: Some(ApiKey { prefix: None, key }),
        ..Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
fn init() -> Configuration {
    let base_path = "http://localhost:5000".to_owned();
    let key = "VerySecretKey".to_owned();

    Configuration {
        base_path,
        api_key: Some(ApiKey { prefix: None, key }),
        ..Default::default()
    }
}

pub struct Config;

impl Config {
    pub fn get() -> &'static Configuration {
        CONFIG.get_or_init(init)
    }
}
