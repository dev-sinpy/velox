use crate::config;
use crate::VeloxError;
use std::env;
use std::path::Path;

pub fn get_asset_path(config: &config::VeloxConfig) -> Result<String, VeloxError> {
    // let config = config::load_config().unwrap();
    if cfg!(target_os = "linux") {
        let arg = env::args().find(|arg| arg == &String::from("debug"));
        if let Some(arg) = arg {
            let asset_path = Path::new(&config.build_dir);
            println!("{:?}", asset_path);
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        } else {
            let asset_path = Path::new("/usr/lib/").join(&config.name).join("web/dist/");
            println!("{:?}", asset_path);
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        }
    } else {
        let asset_path = Path::new("/usr/li/").join(env::var("CARGO_PKG_NAME").unwrap());
        if asset_path.exists() || asset_path.is_dir() {
            Ok(asset_path.to_str().unwrap().to_string())
        } else {
            panic!("could not load assets");
        }
    }
}
