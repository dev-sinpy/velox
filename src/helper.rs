use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::fmt::Display;
use webview_official::Webview;

use custom_error::custom_error;
use notify_rust;
use std::io;

custom_error! {pub VeloxError
    NotificationError{source: notify_rust::error::Error} = "{source}",
    IoError{source: io::Error} = "{source}",
    DialogError{detail: String} = "{detail}",
}

pub fn execute_cmd<T: Serialize>(
    result: Result<T, VeloxError>,
    webview: &mut Webview<'_>,
    success_callback: String,
    error_callback: String,
) {
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

pub enum Response<T> {
    Success(T),
    Error(T),
}

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

pub fn format_callback_result<T: Serialize>(result: T, callback: String) -> String {
    format_callback(callback, serde_json::to_value(result).unwrap())
}
