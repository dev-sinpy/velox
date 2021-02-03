use crate::config;
use crate::VeloxError;
use std::env;
use std::path::Path;

pub fn get_asset_path(config: &config::VeloxConfig) -> Result<String, VeloxError> {
    println!("{:?}", std::env::var_os("CARGO_PKG_NAME"));
    if cfg!(target_os = "linux") {
        let arg = env::args().find(|arg| arg == &String::from("debug"));
        println!("{:?}", env::args());
        if let Some(arg) = arg {
            let asset_path = Path::new(&config.build_dir);
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        } else {
            let asset_path = Path::new("/usr/lib/").join(&config.name).join("web/dist/");
            if asset_path.exists() || asset_path.is_dir() {
                Ok(asset_path.to_str().unwrap().to_string())
            } else {
                panic!("could not load assets");
            }
        }
    } else if cfg!(target_os = "windows"){
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
