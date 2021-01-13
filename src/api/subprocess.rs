use crate::VeloxError;
use std::process::{Child, Command, Stdio};

/// Spawns a new subprocess and returns a process handle.
pub fn exec<T: std::convert::AsRef<std::path::Path>>(
    cmd: &str,
    cwd: T,
    stream_output: bool,
) -> Result<String, VeloxError> {
    if stream_output {
        let mut process = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(cwd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .args(&["/C", cmd])
                .spawn()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        } else {
            Command::new("sh")
                .current_dir(cwd)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .args(&["-c", cmd])
                .spawn()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        };

        match process.try_wait() {
            Ok(Some(_status)) => Ok("".to_string()),
            Ok(None) => {
                let res = process.wait_with_output()?;
                if res.status.success() {
                    Ok(String::from_utf8_lossy(&res.stdout).to_string())
                } else {
                    Ok(String::from_utf8_lossy(&res.stderr).to_string())
                }
            }
            Err(err) => Err(VeloxError::SubProcessError {
                detail: err.to_string(),
            }),
        }
    } else {
        let process = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(cwd)
                .args(&["/C", cmd])
                .output()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        } else {
            Command::new("sh")
                .current_dir(cwd)
                .args(&["-c", cmd])
                .output()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        };

        if process.status.success() {
            Ok(String::from_utf8_lossy(&process.stdout).to_string())
        } else {
            Ok(String::from_utf8_lossy(&process.stderr).to_string())
        }
    }
}
