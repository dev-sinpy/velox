// use custom_error::custom_error;

// // Note the use of braces rather than parentheses.
// custom_error! {CommandError
//     Unknown{code:u8} = "unknown error with code {code}.",
//     Err41            = "Sit by a lake"
// }

// pub fn execute_cmd(args: Vec<String>) {
//     println!("{:?}", args.as_slice());
// }

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::fmt::Display;
use webview_official::Webview;

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
