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
pub mod config;
pub mod events;
pub mod handler;
pub mod plugin;
pub mod server;

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
    TomlError{source: de::Error} = "{source}",
    JSONError{source: serde_json::error::Error} = "{source}",
    CommandError{detail: String} = "{detail}",
    NotificationError{source: notify_rust::error::Error} = "{source}",
    SubProcessError{detail: String} = "{detail}",
    IoError{source: io::Error} = "{source}",
    DialogError{detail: String} = "{detail}",
}

pub type Result<T> = std::result::Result<T, VeloxError>;

// pub fn execute_cmd_async<
//     T: 'static + Serialize + Send,
//     F: 'static + FnOnce() -> std::result::Result<T, VeloxError> + Send,
// >(
//     task: F,
//     proxy: Arc<WindowProxy>,
//     success_callback: String,
//     error_callback: String,
// ) {
//     let pool = threadpool::Builder::new().build();
//     pool.execute(move || {
//         let js = format_callback_result(task(), success_callback, error_callback);
//         proxy.evaluate_script(&js).unwrap();
//     });
// }

/// Executes a given task in a new thread and passes return value
/// to a webview instance to return the data to frontend.
// pub fn execute_cmd<T: Serialize, F: FnOnce() -> std::result::Result<T, VeloxError>>(
//     task: F,
//     proxy: Arc<WindowProxy>,
//     success_callback: String,
//     error_callback: String,
// ) {
//     let js = format_callback_result(task(), success_callback, error_callback);
//     proxy.evaluate_script(&js).unwrap();
// }

// pub fn format_callback<T: Into<JsonValue>, S: AsRef<str> + Display>(
//     function_name: S,
//     arg: T,
// ) -> String {
//     format!(
//       r#"
//       if (window["{fn}"]) {{
//         window["{fn}"]({arg})
//       }} else {{
//         console.warn("[Velox] Couldn't find callback id {fn} in window. This happens when the app is reloaded while Rust is running an asynchronous operation.")
//       }}
//     "#,
//       fn = function_name,
//       arg = arg.into().to_string()
//     )
// }

// pub fn format_callback_result<T: Serialize, E: Display>(
//     result: std::result::Result<T, E>,
//     success_callback: String,
//     error_callback: String,
// ) -> String {
//     match result {
//         Ok(val) => format_callback(success_callback, convert_to_json(val)),
//         Err(err) => format_callback(error_callback, convert_to_json(err.to_string())),
//     }
// }

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
