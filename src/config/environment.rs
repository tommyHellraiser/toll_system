use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use serde::Deserialize;
use tokio::sync::RwLock;
use crate::config::ENV_LOCATION;

lazy_static!{
    static ref ENVIRONMENT: Environment = Environment::new();
}

pub struct Environment {
    inner: RwLock<EnvironmentInner>
}

#[derive(Deserialize)]
struct EnvironmentInner {
    api_ip: String,
    api_port: u16
}

impl Environment {
    fn new() -> Self {
        Self {
            inner: RwLock::new(Self::load_from_json())
        }
    }
    
    fn load_from_json() -> EnvironmentInner {
        let mut reader = File::options()
            .append(false)
            .create(false)
            .read(true)
            .write(false)
            .open(ENV_LOCATION)
            .expect("Failed to open env.json file!");
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).expect("Failed to read env.json content");
        
        serde_json::from_str(&buffer).expect("Failed to deserialize env.json into struct!")
    }
    
    fn instance() -> &'static Self {
        &ENVIRONMENT
    }
    
    //  Getters for all Environment elements
    pub async fn get_api_ip() -> String {
        Self::instance().inner.read().await.api_ip.clone()
    }
    
    pub async fn get_api_port() -> u16 {
        Self::instance().inner.read().await.api_port
    }
}
