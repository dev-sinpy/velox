use crate::VeloxError;
use serde::{Deserialize, Serialize};

/// The `metadata` section of the package configuration.
///
/// # Example Cargo.toml
/// ```
/// [package]
/// name = "..."
///
/// [package.metadata.bundle]
/// identifier = "..."
/// ...other properties from BundleSettings
/// ```

/// The `package` section of the app configuration (read from Cargo.toml).
#[derive(Clone, Debug, Deserialize)]
pub struct PackageSettings {
    /// the package's name.
    pub name: String,
    /// the package's version.
    pub version: String,
    /// the package's description.
    description: String,
    /// the package's homepage.
    homepage: Option<String>,
    /// the package's authors.
    authors: Option<Vec<String>>,
    /// the default binary to run.
    default_run: Option<String>,
}

/// The `workspace` section of the app configuration (read from Cargo.toml).
#[derive(Clone, Debug, Deserialize)]
struct WorkspaceSettings {
    /// the workspace members.
    members: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
struct BinarySettings {
    name: String,
    path: Option<String>,
}

/// The Cargo settings (Cargo.toml root descriptor).
#[derive(Clone, Debug, Deserialize)]
pub struct CargoSettings {
    /// the package settings.
    ///
    pub package: PackageSettings,
    /// the workspace settings.
    ///
    /// it's present if the read Cargo.toml belongs to a workspace root.
    workspace: Option<WorkspaceSettings>,
    /// the binary targets configuration.
    bin: Option<Vec<BinarySettings>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VeloxConfig {
    ///title of the app
    pub name: String,
    pub description: String,
    pub debug: bool,
    pub permissions: Vec<String>,
    pub dev_server_url: String,
    pub package_manager: String,
    pub build_dir: String,
}

pub fn parse_cargo_config() -> Result<CargoSettings, VeloxError> {
    use std::fs;

    let cargo_config = fs::read_to_string("./Cargo.toml")?;
    let config_toml: CargoSettings = toml::from_str(&cargo_config)?;

    Ok(config_toml)
}

pub fn parse_config(config: &str) -> Result<VeloxConfig, VeloxError> {
    let config_json: VeloxConfig = serde_json::from_str(config)?;

    Ok(config_json)
}
