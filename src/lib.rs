//! A GUI framework that focuses on simplicity, performance and ease of use.
//! It allows you to build cross platform native apps using your favourite web framework.
//! Velox uses rust and wry under the hood which helps to keep binary size to absolute minimal.
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
    /// If something goes wrong these errors will be returned
    pub VeloxError
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

pub type Result<T> = std::result::Result<T, VeloxError>;

/// Response data to be send back to javascript
pub enum Response<T> {
    /// Successful response with result
    Success(T),
    /// Error response with details about the error
    Error(T),
}

/// Converts a data structure to JSON so that the reult can be passed to the frontend
pub fn convert_into_json<T: Serialize>(res: Response<T>) -> serde_json::Value {
    match res {
        Response::Success(data) => json!({
            "result": data,
        }),
        Response::Error(msg) => json!({
            "error": msg,
        }),
    }
}

/// Converts a data structure to JSON so that the reult can be passed to the frontend
pub fn convert_to_json<T: Serialize>(res: T) -> wry::Value {
    json!({
        "result": res,
    })
}
