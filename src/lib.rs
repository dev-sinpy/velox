//! Velox is a framework that focuses on simplicity, performance and ease of use.
//! It allows you to build cross platform native apps using web-technology.
//! Velox uses rust and webview under the hood which helps to keep binary size to absolute minimal.
//!
//! **Note:** This documentation is for velox_core which only includes core modules
//! and some helper functions. If you're looking for the complete documention of the framework
//! go to [velox github](https://github.com/dev-sinpy/velox) page.

pub mod api;
pub mod app;
pub mod assets;
pub mod cmd;
pub mod config;
pub mod handler;
pub mod server;

pub use crate::api::fs::file_system;
pub use app::AppBuilder;
pub use config::VeloxConfig;

use confy::ConfyError;
use serde::Serialize;
use serde_json::json;
use serde_json::Value as JsonValue;
use std::fmt::{Debug, Display};
use std::io;
use toml::de;
use webview_official::Webview;

use custom_error::custom_error;

custom_error! {
    /// If something goes wrong these errors will be returned
    pub VeloxError
    ConfigError{source: ConfyError} = "{source}",
    TomlError{source: de::Error} = "{source}",
    CommandError{source: serde_json::error::Error} = "{source}",
    NotificationError{source: notify_rust::error::Error} = "{source}",
    SubProcessError{detail: String} = "{detail}",
    IoError{source: io::Error} = "{source}",
    DialogError{detail: String} = "{detail}",
}

/// Executes a given task in a new thread and passes return value
/// to a webview instance to return the data to frontend.
pub fn execute_cmd<T: Serialize, F: FnOnce() -> Result<T, VeloxError>>(
    task: F,
    webview: &mut Webview<'_>,
    success_callback: String,
    error_callback: String,
) {
    let js = format_callback_result(task(), success_callback, error_callback);
    webview.dispatch(move |w| w.eval(js.as_str()));
}

pub fn format_callback<T: Into<JsonValue>, S: AsRef<str> + Display>(
    function_name: S,
    arg: T,
) -> String {
    format!(
      r#"
      if (window["{fn}"]) {{
        window["{fn}"]({arg})
      }} else {{
        console.warn("[Ezgui] Couldn't find callback id {fn} in window. This happens when the app is reloaded while Rust is running an asynchronous operation.")
      }}
    "#,
      fn = function_name,
      arg = arg.into().to_string()
    )
}

pub fn format_callback_result<T: Serialize, E: Display>(
    result: Result<T, E>,
    success_callback: String,
    error_callback: String,
) -> String {
    match result {
        Ok(val) => format_callback(success_callback, convert_to_json(Response::Success(val))),
        Err(err) => format_callback(
            error_callback,
            convert_to_json(Response::Error(err.to_string())),
        ),
    }
}

/// Response data to be send back to javascript
pub enum Response<T> {
    /// Successful response with result
    Success(T),
    /// Error response with details about the error
    Error(T),
}

/// Converts a data structure to JSON so that the reult can be passed to the frontend
pub fn convert_to_json<T: Serialize>(res: Response<T>) -> String {
    match res {
        Response::Success(data) => json!({
            "result": data,
        })
        .to_string(),
        Response::Error(msg) => json!({
            "error": msg,
        })
        .to_string(),
    }
}
