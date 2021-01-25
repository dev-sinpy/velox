// use crate::assets;
use crate::VeloxError;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct VeloxConfig {
//     ///title of the app
//     pub title: String,
//     pub description: String,
//     pub debug: bool,
//     pub permissions: Vec<String>,
//     pub dev_server_url: String,
//     pub package_manager: String,
//     pub build_dir: String,
// }

// impl std::default::Default for VeloxConfig {
//     fn default() -> Self {
//         Self {
//             title: String::from(""),
//             description: String::from(""),
//             debug: true,
//             permissions: vec![],
//             dev_server_url: String::from("0.0.0.0:8888"),
//             package_manager: String::from("npm"),
//             build_dir: String::from("web/dist/"),
//         }
//     }
// }

// pub fn load_config() -> Result<VeloxConfig, VeloxError> {
//     let config: VeloxConfig = confy::load_path("velox-config.toml")?;
//     Ok(config)
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct VeloxConfig {
    ///title of the app
    pub title: String,
    pub description: String,
    pub debug: bool,
    pub permissions: Vec<String>,
    pub dev_server_url: String,
    pub package_manager: String,
    pub build_dir: String,
}

pub fn parse_config(config: &str) -> Result<VeloxConfig, VeloxError> {
    use std::env;
    use std::fs;

    // let config = fs::read_to_string(path)?;
    let config_json: VeloxConfig = serde_json::from_str(config)?;

    Ok(config_json)
}
