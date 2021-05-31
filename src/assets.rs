use crate::config;
use crate::VeloxError;
use std::env;
use std::path::Path;

// Returns the path of where the assets are located
pub fn get_asset_path(config: &config::VeloxConfig) -> Result<String, VeloxError> {
    if cfg!(target_os = "linux") {
        // check if app is being run by cargo
        let arg = env::args().find(|arg| arg.contains("target"));
        if let Some(_arg) = arg {
            let asset_path = Path::new(&config.build_dir); // get asset path from config file

            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        } else {
            let config_toml = config::parse_cargo_config()?; // parse Cargo.toml file of app

            // user is running bundled app, so only serve bundled assets
            let asset_path = Path::new("/usr/lib/")
                .join(&config_toml.package.name)
                .join("dist/");
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        }
    } else if cfg!(target_os = "windows") {
        let arg = env::args().find(|arg| arg.contains("target"));
        if let Some(_arg) = arg {
            let asset_path = Path::new(&config.build_dir);
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        } else {
            let mut asset_path = env::current_exe()?;
            asset_path.pop();
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        }
    } else {
        unimplemented!();
    }
}
