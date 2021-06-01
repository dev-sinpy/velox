//! A GUI framework that focuses on simplicity, performance and ease of use.
//! It allows you to build cross platform native apps using your favourite web framework.
//! Velox uses [wry](https://github.com/tauri-apps/wry) under the hood which helps to keep binary size to absolute minimal.
//!
//! **Note:** This documentation is for velox_core which only includes core modules
//! and some helper functions. If you're looking for the complete documention of the framework
//! go to [velox github](https://github.com/dev-sinpy/velox) page.

pub mod api;
pub mod app;
pub mod assets;
pub mod config;
pub mod events;
pub mod handler;
pub mod plugin;
pub mod server;
pub mod window;

pub use crate::api::fs::file_system;
pub use app::AppBuilder;
pub use config::VeloxConfig;
pub use serde_json::json;

use serde::Serialize;
use std::fmt::Debug;
use std::io;
use toml::de;

use custom_error::custom_error;

custom_error! {
    /// Errors returned by velox
    pub Error
    WryError{source: wry::Error} = "{source}",
    EventLoopClosed{source: wry::application::event_loop::EventLoopClosed<events::Event>} = "{source}",
    TomlError{source: de::Error} = "{source}",
    JSONError{source: serde_json::error::Error} = "{source}",
    CommandError{detail: String} = "{detail}",
    NotificationError{source: notify_rust::error::Error} = "{source}",
    SubProcessError{detail: String} = "{detail}",
    IoError{source: io::Error} = "{source}",
    DialogError{detail: String} = "{detail}",
}

pub type Result<T> = std::result::Result<T, Error>;

/// Describes response data that will be sent back to javascript
pub enum Response<T: Serialize> {
    /// Successful response with result
    Success(T),
    /// Error response with details about the error
    Error(T),
}

impl<T: Serialize> Response<T> {
    /// Converts a response to JSON
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Response::Success(data) => json!({
                "result": data,
            }),
            Response::Error(msg) => json!({
                "error": msg,
            }),
        }
    }

    // converts an error message to JSON
    pub fn from_error(err: T) -> serde_json::Value {
        json!({
            "error": err,
        })
    }
}

/// Converts a data structure to JSON so that the reult can be passed to the frontend
pub fn convert_to_json<T: Serialize>(res: T) -> wry::Value {
    json!({
        "result": res,
    })
}
