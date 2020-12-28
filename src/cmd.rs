use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt::Display;
use webview_official::Webview;

pub fn handle_cmd(webview: &mut Webview<'_>, arg: &str) -> Option<()> {
    // function for calling appropiate Api from a given command.
    // returns None if command is not recognised
    use crate::api::fs::utils::*;
    use file_system::FilePath;
    use Cmd::*;

    let command: Cmd = serde_json::from_str(arg).unwrap();

    match command {
        Notification(noti) => match noti {
            Notify::ShowNotification {
                summary,
                body,
                timeout,
                success_callback,
                error_callback,
            } => {
                let result = show_notification(summary, body, timeout);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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
        },

        FileSystem(fs) => match fs {
            FsApi::ReadDir {
                path,
                success_callback,
                error_callback,
            } => {
                let result = file_system::read_dir(path);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success(val)),
                            success_callback,
                        );
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

            FsApi::CreateDir {
                path,
                success_callback,
                error_callback,
            } => {
                let result = file_system::create_dir(path);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::CreateFile {
                path,
                success_callback,
                error_callback,
            } => {
                let result = file_system::create_file(path);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::RemoveFile {
                path,
                success_callback,
                error_callback,
            } => {
                let result = file_system::remove_file(path);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::RemoveDir {
                path,
                success_callback,
                error_callback,
            } => {
                let result = file_system::remove_dir(path);
                match result {
                    Ok(val) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::CopyFile {
                from,
                to,
                success_callback,
                error_callback,
            } => {
                let result = file_system::copy_file(from, to);
                match result {
                    Ok(()) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::RenameFile {
                from,
                to,
                success_callback,
                error_callback,
            } => {
                let result = file_system::rename_file(from, to);
                match result {
                    Ok(()) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Success("success")),
                            success_callback,
                        );
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

            FsApi::OpenDialog {
                multiple,
                filter,
                success_callback,
                error_callback,
            } => {
                let result = file_system::open_dialog(multiple, filter);

                if let Some(val) = result {
                    match val {
                        FilePath::Single(path) => {
                            let callback_string = format_callback_result(
                                convert_to_json(Response::Success(path)),
                                success_callback,
                            );
                            webview.dispatch(move |w| w.eval(callback_string.as_str()));
                        }
                        FilePath::Multiple(path) => {
                            let callback_string = format_callback_result(
                                convert_to_json(Response::Success(path)),
                                success_callback,
                            );
                            webview.dispatch(move |w| w.eval(callback_string.as_str()));
                        }
                    }
                } else {
                    let callback_string = format_callback_result(
                        convert_to_json(Response::Error("User did not select any file.")),
                        error_callback,
                    );
                    webview.dispatch(move |w| w.eval(callback_string.as_str()));
                }
            }

            FsApi::SelectFolder {
                success_callback,
                error_callback,
            } => {
                let result = file_system::select_folder();
                if let Some(val) = result {
                    let callback_string = format_callback_result(
                        convert_to_json(Response::Success(val)),
                        success_callback,
                    );
                    webview.dispatch(move |w| w.eval(callback_string.as_str()));
                } else {
                    let callback_string = format_callback_result(
                        convert_to_json(Response::Error("User did not select any folder.")),
                        error_callback,
                    );
                    webview.dispatch(move |w| w.eval(callback_string.as_str()));
                }
            }
            _ => {}
        },
        _ => {}
    };
    Some(())
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Callback(String);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Cmd {
    FileSystem(FsApi),
    Notification(Notify),
    // Execute()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Notify {
    ShowNotification {
        summary: String,
        body: String,
        timeout: i32,
        success_callback: String,
        error_callback: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FsApi {
    ReadDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    CopyFile {
        from: String,
        to: String,
        success_callback: String,
        error_callback: String,
    },
    RenameFile {
        from: String,
        to: String,
        success_callback: String,
        error_callback: String,
    },
    OpenDialog {
        multiple: bool,
        filter: Option<Vec<String>>,
        success_callback: String,
        error_callback: String,
    },
    OpenFile {
        path: String,
        format: Option<Vec<String>>,
    },
    CreateDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    CreateFile {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    RemoveDir {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    RemoveFile {
        path: String,
        success_callback: String,
        error_callback: String,
    },
    OpenMultipleFile {
        path: String,
        format: Option<Vec<String>>,
    },
    OpenDir {
        path: String,
    },
    SelectFolder {
        success_callback: String,
        error_callback: String,
    },
}

use serde_json::json;

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
