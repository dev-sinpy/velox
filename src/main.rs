mod api;
mod app;
mod cmd;

use crate::api::fs::file_system;
use crate::api::notification::show_notification;
use app::AppBuilder;

fn main() {
    // show_notification("hey", "this is noti", 4000);
    let app = AppBuilder::new("http://localhost:1234")
        .invoke_handler(|_web, arg| {
            println!("{:?}", arg);
            Ok(())
        })
        .build();
    app.run();
}
