use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::api::subprocess;
use crate::api::window;
use crate::cmd::*;
use crate::{convert_to_json, execute_cmd_async, VeloxError};

use std::sync::Arc;

/// A command handler which passes commands from webview to the velox-api
pub fn handle_cmd(proxy: Arc<wry::WindowProxy>, arg: &str) -> Result<(), VeloxError> {
    use crate::cmd::Cmd::*;

    let command: Cmd = serde_json::from_str(arg).unwrap();

    match command {
        // Window(window_proxy) => match window_proxy {
        //     WindowProxy::SetTitle {
        //         title,
        //         success_callback,
        //         error_callback,
        //     } => {
        //         let new_proxy = proxy.clone();
        //         execute_cmd_async(
        //             move || window::set_title(title, new_proxy),
        //             proxy,
        //             success_callback,
        //             error_callback,
        //         );
        //     }

        //     WindowProxy::SetFullscreen {
        //         fullscreen,
        //         success_callback,
        //         error_callback,
        //     } => {
        //         let new_proxy = proxy.clone();
        //         execute_cmd_async(
        //             move || window::set_fullscreen(fullscreen, new_proxy),
        //             proxy,
        //             success_callback,
        //             error_callback,
        //         );
        //     }
        // },
        Notification(noti) => match noti {
            Notify::ShowNotification {
                summary,
                body,
                timeout,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || show_notification(summary, body, timeout),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }
        },

        SubProcess(process) => match process {
            Process::Exec {
                cmd,
                cwd,
                stream_output,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || subprocess::exec(cmd, cwd, stream_output),
                    proxy,
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
                execute_cmd_async(
                    move || file_system::read_dir(path),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::ReadTextFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::read_text_file(path),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::CreateDir {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::create_dir(path),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::CreateFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::create_file(path),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::RemoveFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::remove_file(path),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::RemoveDir {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::remove_dir(path),
                    proxy,
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
                execute_cmd_async(
                    move || file_system::copy_file(from, to),
                    proxy,
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
                execute_cmd_async(
                    move || file_system::rename_file(from, to),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

            // FsApi::OpenDialog {
            //     multiple,
            //     filter,
            //     success_callback,
            //     error_callback,
            // } => {
            //     execute_cmd_async(
            //         move || file_system::open_dialog(multiple, filter),
            //         proxy,
            //         success_callback,
            //         error_callback,
            //     );
            // }
            FsApi::SelectFolder {
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    file_system::select_folder,
                    proxy,
                    success_callback,
                    error_callback,
                );
            }
            FsApi::SaveFile {
                path,
                content,
                mode,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::save_file(path, &content[..], mode),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }
            _ => {}
        },

        _ => {}
    };
    Ok(())
}

/// A command handler which passes commands from webview to the velox-api
pub fn call_func(
    proxy: wry::WindowProxy,
    func_name: String,
    params: Vec<wry::Value>,
) -> Result<wry::Value, VeloxError> {
    match func_name.as_str() {
        "add_window" => {
            let res = window::add_window(
                serde_json::from_str(&params[0].to_string())?,
                serde_json::from_str(&params[1].to_string())?,
                proxy,
            )?;
            Ok(convert_to_json(res))
        }

        "set_title" => {
            window::set_title(serde_json::from_str(&params[0].to_string())?, proxy)?;
            Ok(convert_to_json("success"))
        }

        "set_fullscreen" => {
            window::set_fullscreen(params[0].as_bool().unwrap(), proxy)?;
            Ok(convert_to_json("success"))
        }

        "show_notification" => {
            let res = show_notification(
                serde_json::from_str(&params[0].to_string())?,
                serde_json::from_str(&params[1].to_string())?,
                params[2].as_i64().unwrap() as i32,
            )?;
            Ok(convert_to_json(res))
        }

        "exec" => {
            let res = subprocess::exec(
                serde_json::from_str::<String>(&params[0].to_string())?,
                serde_json::from_str::<String>(&params[1].to_string())?,
                params[2].as_bool().unwrap(),
            )?;
            Ok(convert_to_json(res))
        }

        "read_dir" => {
            let res = file_system::read_dir(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "read_text_file" => {
            let res = file_system::read_text_file(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "create_dir" => {
            let res = file_system::create_dir(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "create_file" => {
            let res = file_system::create_file(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "remove_file" => {
            let res = file_system::remove_file(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "remove_dir" => {
            let res = file_system::remove_dir(serde_json::from_str(&params[0].to_string())?)?;
            Ok(convert_to_json(res))
        }

        "copy_file" => {
            let res = file_system::copy_file(
                serde_json::from_str(&params[0].to_string())?,
                serde_json::from_str(&params[1].to_string())?,
            )?;
            Ok(convert_to_json(res))
        }

        "rename_file" => {
            let res = file_system::rename_file(
                serde_json::from_str(&params[0].to_string())?,
                serde_json::from_str(&params[1].to_string())?,
            )?;
            Ok(convert_to_json(res))
        }

        "open_dialog" => {
            let res = file_system::open_dialog(params[0].as_bool().unwrap())?;
            Ok(convert_to_json(res))
        }

        "select_folder" => {
            let res = file_system::select_folder()?;
            Ok(convert_to_json(res))
        }

        "save_file" => {
            let res = file_system::save_file(
                serde_json::from_str::<String>(&params[0].to_string())?,
                serde_json::from_str::<Vec<u8>>(&params[1].to_string())?.as_slice(),
                serde_json::from_str(&params[2].to_string())?,
            )?;
            Ok(convert_to_json(res))
        }

        _ => Err(VeloxError::CommandError {
            detail: "Invalid command".to_string(),
        }),
    }
}
