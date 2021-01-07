use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::cmd::*;
use crate::{execute_cmd, VeloxError};

use std::path::Path;
use webview_official::Webview;

/// A command handler which passes commands from webview to the velox-api
pub fn handle_cmd(webview: &mut Webview<'_>, arg: &str) -> Result<(), VeloxError> {
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

            FsApi::ReadTextFile {
                path,
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    || file_system::read_text_file(path),
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
                execute_cmd(
                    || file_system::open_dialog(multiple, filter),
                    webview,
                    success_callback,
                    error_callback,
                );
            }

            FsApi::SelectFolder {
                success_callback,
                error_callback,
            } => {
                execute_cmd(
                    file_system::select_folder,
                    webview,
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
                let path_buf = Path::new(&path);
                execute_cmd(
                    || file_system::save_file(path_buf, &content[..], mode),
                    webview,
                    success_callback,
                    error_callback,
                );
            }
            _ => {}
        },
    };
    Ok(())
}
