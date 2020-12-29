mod api;
mod app;
mod cmd;
mod handler;
mod helper;

use crate::api::fs::file_system;
use app::AppBuilder;
use custom_error::custom_error;
use notify_rust;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Debug, Display};
use std::io;
use webview_official::Webview;

custom_error! {pub VeloxError
    NotificationError{source: notify_rust::error::Error} = "{source}",
    IoError{source: io::Error} = "{source}"
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

pub fn format_callback_result<T: Serialize, E: Serialize>(
    result: Result<T, E>,
    callback: String,
) -> String {
    let res = match result {
        Ok(val) => format_callback(callback, serde_json::to_value(val).unwrap()),
        Err(val) => "".to_string(),
    };

    res
}
