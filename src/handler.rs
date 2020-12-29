use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::cmd::*;
use crate::helper::*;
use file_system::FilePath;
use webview_official::Webview;

pub fn handle_cmd(webview: &mut Webview<'_>, arg: &str) -> Option<()> {
    // function for calling appropiate Api from a given command.
    // returns None if command is not recognised

    use crate::cmd::Cmd::*;

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
                execute_cmd(
                    || show_notification(summary, body, timeout),
                    webview,
                    success_callback,
                    error_callback,
                );
            }
        },

        FileSystem(fs) => match fs {
            FsApi::ReadDir {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::read_dir(path),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::CreateDir {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::create_dir(path),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::CreateFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::create_file(path),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::RemoveFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::remove_file(path),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::RemoveDir {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::remove_dir(path),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::CopyFile {
                from,
                to,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::copy_file(from, to),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::RenameFile {
                from,
                to,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::rename_file(from, to),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::OpenDialog {
                multiple,
                filter,
                success_callback,
                error_callback,
            } => {
                let result = file_system::open_dialog(multiple, filter);

                match result {
                    Ok(val) => match val {
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
                    },
                    Err(err) => {
                        let callback_string = format_callback_result(
                            convert_to_json(Response::Error(err.to_string())),
                            error_callback,
                        );
                        webview.dispatch(move |w| w.eval(callback_string.as_str()));
                    }
                }
            }

            FsApi::SelectFolder {
                success_callback,
                error_callback,
            } => {
                let result = file_system::select_folder();

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
            _ => {}
        },
    };
    Some(())
}
