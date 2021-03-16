use std::fs;
// use std::io::Cursor;
use std::path::Path;
use std::thread::spawn;

use crate::assets;
use crate::config;
use tiny_http::{self, Request, Response};

// enum Route<F> {
//     Homepage { req: Request, func: F },
//     Error404 { req: Request, func: F },
// }

// enum ResponseType {
//     Success(ResponseKind),
//     Error(ResponseKind),
// }

// enum ResponseKind {
//     File(Response<fs::File>),
//     Text(Response<Cursor<Vec<u8>>>),
// }

// impl Route<Box<dyn FnOnce() -> ResponseType>> {

//     fn _respond(self) {
//         match self {
//             Route::Homepage { req, func } => match func() {
//                 ResponseType::Success(response_type) => match response_type {
//                     ResponseKind::File(res) => {
//                         req.respond(res).unwrap();
//                     }
//                     ResponseKind::Text(res) => {
//                         req.respond(res).unwrap();
//                     }
//                 },
//                 ResponseType::Error(response_type) => match response_type {
//                     ResponseKind::File(res) => {
//                         req.respond(res).unwrap();
//                     }
//                     ResponseKind::Text(res) => {
//                         req.respond(res).unwrap();
//                     }
//                 },
//             },

//             Route::Error404 { req, func } => match func() {
//                 ResponseType::Error(response_type) => match response_type {
//                     ResponseKind::File(res) => {
//                         req.respond(res).unwrap();
//                     }
//                     ResponseKind::Text(res) => {
//                         req.respond(res).unwrap();
//                     }
//                 },

//                 _ => panic!("Error route cannot respond with any other response type"),
//             },
//         }
//     }
// }

fn handle_req(req: Request, config: &config::VeloxConfig) {
    match req.url() {
        "/" => home_screen(req, config),

        _ => serve_assets(req, config),
    };
}

pub fn spawn_server(addrs: &str, config: config::VeloxConfig) {
    let server = tiny_http::Server::http(addrs).unwrap();
    println!("Server spawned on url: {:?}", addrs);
    spawn(move || loop {
        match server.recv() {
            Ok(req) => handle_req(req, &config),
            Err(e) => {
                println!("error: {}", e);
                break;
            }
        };
    });
}

fn home_screen(req: Request, config: &config::VeloxConfig) {
    let asset = Path::new(&assets::get_asset_path(config).unwrap()).join("index.html");
    let file = fs::read(asset).unwrap();
    let html_file = String::from_utf8_lossy(&file);
    let response = Response::from_string(html_file)
        .with_header(
            "Content-type: text/html"
                .parse::<tiny_http::Header>()
                .unwrap(),
        )
        .with_header(
            "Access-Control-Allow-Origin: *"
                .parse::<tiny_http::Header>()
                .unwrap(),
        );

    req.respond(response).unwrap();
}

fn serve_assets(req: Request, config: &config::VeloxConfig) {
    let path = Path::new(req.url()).strip_prefix("/").unwrap();
    let asset = Path::new(&assets::get_asset_path(config).unwrap()).join(path);
    let file = fs::read(asset);
    if let Ok(file) = file {
        let response = Response::from_data(file)
            .with_header(
                format!("Content-type: {}", get_mime_type(path))
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            )
            .with_header(
                "Access-Control-Allow-Origin: *"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
        req.respond(response).unwrap();
    } else {
        let response = Response::empty(404);
        req.respond(response).unwrap();
    }
}

fn get_mime_type(path: &Path) -> &str {
    match path.extension().unwrap().to_str().unwrap() {
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "txt" => "text/plain",
        "json" => "application/json",
        "png" => "image/png",
        "jpeg" | "jpg" => "image/jpeg",
        "ico" => "image/vnd.microsoft.icon",
        "pdf" => "application/pdf",
        "gif" => "image/gif",
        "mpeg" => "video/mpeg",
        "mp3" => "audio/mpeg",
        "ttf" => "font/ttf",
        _ => "application/octet-stream",
    }
}
