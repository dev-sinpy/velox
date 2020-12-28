use std::process::Command;

struct Cmd {
    name: String,
    args: Option<Vec<String>>,
}

fn execute(cmd: &str, options: Options) {
    if options.stream_output {
        let mut process = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd])
                // .output()
                .spawn()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                // .output()
                .expect("failed to execute process")
        };
        let output = process.wait_with_output().expect("failed to wait on child");
        println!("{:?}", output.stdout);
    } else {
        let mut process = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", cmd])
                // .output()
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                // .output()
                .expect("failed to execute process")
        };

        println!("{:?}", process.stdout);
    }
}
