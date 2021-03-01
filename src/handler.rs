use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::api::subprocess;
use crate::cmd::*;
use crate::{execute_cmd_async, VeloxError};

use std::sync::Arc;
use wry::WindowProxy;

/// A command handler which passes commands from webview to the velox-api
pub fn handle_cmd(proxy: Arc<WindowProxy>, arg: &str) -> Result<(), VeloxError> {
    use crate::cmd::Cmd::*;

    let command: Cmd = serde_json::from_str(arg)?;

    match command {
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
                    move || subprocess::exec(&cmd, cwd, stream_output),
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

            FsApi::OpenDialog {
                multiple,
                filter,
                success_callback,
                error_callback,
            } => {
                execute_cmd_async(
                    move || file_system::open_dialog(multiple, filter),
                    proxy,
                    success_callback,
                    error_callback,
                );
            }

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
    };
    Ok(())
}
