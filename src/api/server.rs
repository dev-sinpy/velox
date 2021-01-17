use tiny_http;

use std::sync::Arc;
use std::thread;

fn home_screen() {}

fn splash_screen() {}

fn serve() {
    let server = tiny_http::Server::http("0.0.0.0:8000").unwrap();
}
