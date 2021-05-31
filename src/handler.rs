use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::api::subprocess;
use crate::api::window;
use crate::{convert_to_json, VeloxError};

/// A command handler which passes commands from webview to the velox-api
pub fn call_func(
    // proxy: wry::WindowProxy,
    func_name: String,
    params: Vec<wry::Value>,
) -> Result<wry::Value, VeloxError> {
    match func_name.as_str() {
        // "add_window" => {
        //     let res = window::add_window(
        //         serde_json::from_str(&params[0].to_string())?,
        //         serde_json::from_str(&params[1].to_string())?,
        //         proxy,
        //     )?;
        //     Ok(convert_to_json(res))
        // }

        // "set_title" => {
        //     window::set_title(serde_json::from_str(&params[0].to_string())?, proxy)?;
        //     Ok(convert_to_json("success"))
        // }

        // "set_fullscreen" => {
        //     window::set_fullscreen(params[0].as_bool().unwrap(), proxy)?;
        //     Ok(convert_to_json("success"))
        // }
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
