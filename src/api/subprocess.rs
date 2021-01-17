use crate::VeloxError;
use std::net::TcpListener;
use std::process::{Command, Stdio};
use threadpool::ThreadPool;
use tungstenite::server::accept;

/// Spawns a new subprocess and returns a process handle.
#[allow(clippy::never_loop)]
pub fn exec<T: std::convert::AsRef<std::path::Path>>(
    cmd: &str,
    cwd: T,
    stream_output: bool,
) -> Result<String, VeloxError> {
    use std::io::BufRead;

    if stream_output {
        let child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .current_dir(cwd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .args(&["/C", cmd])
                .spawn()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        } else {
            Command::new("sh")
                .current_dir(cwd)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .args(&["-c", cmd])
                .spawn()
                .unwrap_or_else(|_| panic!("SubProcessError: Failed to run command `{}`", cmd))
        };
        let pool = ThreadPool::new(1);
        pool.execute(move || {
            let child_stdout = child.stdout.unwrap();
            let reader = std::io::BufReader::new(child_stdout);
            let server = TcpListener::bind("127.0.0.1:8888").unwrap();
            for stream in server.incoming() {
                let mut websocket = accept(stream.unwrap()).unwrap();
                // let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                for line in reader.lines() {
                    websocket
                        .write_message(tungstenite::Message::Text(line.unwrap()))
                        .unwrap();
                }
                break;
            }
        });

        Ok(String::from("ws://localhost:8888"))
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
