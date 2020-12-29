mod api;
mod app;
mod cmd;
mod handler;
mod helper;

use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use crate::handler::handle_cmd;
// use crate::helper::execute_cmd;
use app::AppBuilder;

fn main() {
    // show_notification("hey", "this is noti", 4000);
    let args: Vec<String> = vec!["dev".to_string(), "is".to_string()];
    // execute_cmd(args);
    let app = AppBuilder::new("http://localhost:1234")
        .invoke_handler(|_web, arg| {
            println!("{:?}", arg);
            Ok(())
        })
        .build();
    app.run();
}
