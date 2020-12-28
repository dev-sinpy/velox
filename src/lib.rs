mod api;
mod app;
mod cmd;

use crate::api::fs::file_system;
use app::AppBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::{Debug, Display};
use webview_official::Webview;

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
