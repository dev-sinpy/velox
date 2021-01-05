//! Velox is a framework that focuses on simplicity, performance and ease of use.
//! It allows you to build cross platform native apps using web-technology.
//! Velox uses rust and webview under the hood which helps to keep binary size to absolute minimal.
//!
//! **Note:** This documentation is for velox_core which only includes core modules
//! and some helper functions. If you're looking for the complete documention of the framework
//! go to [velox github](https://github.com/dev-sinpy/velox) page.

pub mod api;
pub mod app;
pub mod cmd;
pub mod config;
pub mod handler;
pub mod helper;

pub use crate::api::fs::file_system;
pub use app::AppBuilder;
pub use config::VeloxConfig;

use confy::ConfyError;
use serde::Serialize;
use serde_json::Value as JsonValue;
use std::fmt::{Debug, Display};
use std::io;
use webview_official::Webview;

use custom_error::custom_error;
use notify_rust;
use serde_json;

/// If something goes wrong these errors will be returned
custom_error! {pub VeloxError
    ConfigError{source: ConfyError} = "{source}",
    CommandError{source: serde_json::error::Error} = "{source}",
    NotificationError{source: notify_rust::error::Error} = "{source}",
    IoError{source: io::Error} = "{source}",
    DialogError{detail: String} = "{detail}",
}

fn execute_cmd<T: Into<JsonValue> + Serialize>(
    webview: &mut Webview<'_>,
    result: Result<T, String>,
    success_callback: String,
    error_callback: String,
) {
    let js = format_callback_result(result, success_callback);
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

pub fn format_callback_result<T: Serialize, E: Serialize + Display>(
    result: Result<T, E>,
    callback: String,
) -> String {
    let res = match result {
        Ok(val) => format_callback(callback, serde_json::to_value(val).unwrap()),
        Err(err) => err.to_string(),
    };

    res
}
