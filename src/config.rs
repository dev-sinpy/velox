use crate::VeloxError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VeloxConfig {
    ///title of the app
    pub title: String,
    pub description: String,
    pub debug: bool,
    pub permissions: Vec<String>,
    pub dev_server_url: String,
}

impl ::std::default::Default for VeloxConfig {
    fn default() -> Self {
        Self {
            title: String::from("None"),
            description: String::from("None"),
            debug: true,
            permissions: vec![],
            dev_server_url: String::from("http://localhost:8000"),
        }
    }
}

pub fn load_config() -> Result<VeloxConfig, VeloxError> {
    let config: VeloxConfig = confy::load_path("velox-config.toml")?;
    Ok(config)
}
