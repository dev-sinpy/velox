use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::fmt::Display;
use webview_official::Webview;

use custom_error::custom_error;
use notify_rust;
use serde_json;
use std::io;

/// If something goes wrong these errors will be returned
custom_error! {pub VeloxError
    CommandError{source: serde_json::error::Error} = "{source}",
    NotificationError{source: notify_rust::error::Error} = "{source}",
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
    let result = task();
    match result {
        Ok(val) => {
            let callback_string =
                format_callback_result(convert_to_json(Response::Success(val)), success_callback);
            webview.dispatch(move |w| w.eval(callback_string.as_str()));
        }
        Err(err) => {
            let callback_string = format_callback_result(
                convert_to_json(Response::Error(err.to_string())),
                error_callback,
            );
            webview.dispatch(move |w| w.eval(callback_string.as_str()));
        }
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

/// Formats a callback to a valid JS expression
pub fn format_callback<T: Into<JsonValue>, S: AsRef<str> + Display>(
    function_name: S,
    arg: T,
) -> String {
    format!(
      r#"
      if (window["{fn}"]) {{
        window["{fn}"]({arg})
      }} else {{
        console.warn("[Velox] Couldn't find callback id {fn} in window. This happens when the app is reloaded while Rust is running an asynchronous operation.")
      }}
    "#,
      fn = function_name,
      arg = arg.into().to_string()
    )
}

/// Calls the format_callback fuction with a task result and a callback.
/// If a task result was Ok then call a success callback
/// else call an error callback
pub fn format_callback_result<T: Serialize>(result: T, callback: String) -> String {
    format_callback(callback, serde_json::to_value(result).unwrap())
}
