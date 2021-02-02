use crate::handler::handle_cmd;
use crate::{config, server, VeloxError};

use std::path::Path;
use webview_official::{SizeHint, Webview, WebviewBuilder};

pub type InvokeHandler = Box<dyn FnMut(&mut Webview<'_>, &str) -> Result<(), String>>;

/// The application runner.
pub struct App {
    pub name: &'static str,
    pub debug: bool,
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
    /// Url of the local server where frontend is hosted
    pub url: &'static str,
}

impl App {
    /// Runs the app until it finishes.
    pub fn run(mut self) -> Result<(), String> {
        let mut webview = build_webview(&mut self).unwrap();
        // run the webview
        webview.run();

        Ok(())
    }

    /// Runs the invoke handler if defined.
    /// Returns whether the message was consumed or not.
    /// The message is considered consumed if the handler exists and returns an Ok Result.
    pub fn run_invoke_handler(
        &mut self,
        webview: &mut Webview<'_>,
        arg: &str,
    ) -> Result<bool, String> {
        if let Some(ref mut invoke_handler) = self.invoke_handler {
            invoke_handler(webview, arg).map(|_| true)
        } else {
            Ok(false)
        }
    }
}

/// The App builder.
#[derive(Default)]
pub struct AppBuilder {
    pub name: &'static str,
    pub debug: bool,
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
    /// Url of the local server where frontend is hosted
    pub url: &'static str,
}

impl AppBuilder {
    /// Creates a new App builder.

    pub fn from_config(config: String) -> Self {
        use portpicker::pick_unused_port;

        let config = config::parse_config(&config).unwrap();

        if config.debug {
            Self {
                name: Box::leak(config.name.into_boxed_str()),
                debug: config.debug,
                invoke_handler: None,
                url: Box::leak(config.dev_server_url.into_boxed_str()),
            }
        } else {
            let port = pick_unused_port().expect("no unused port");
            let url = format!("127.0.0.1:{}", port);
            server::spawn_server(&url, config.clone());
            Self {
                name: Box::leak(config.name.into_boxed_str()),
                debug: config.debug,
                invoke_handler: None,
                url: Box::leak(Box::new("http://".to_owned() + &url)),
            }
        }
    }

    /// Defines the JS message handler callback.
    pub fn invoke_handler<F: FnMut(&mut Webview<'_>, &str) -> Result<(), String> + 'static>(
        mut self,
        invoke_handler: F,
    ) -> Self {
        self.invoke_handler = Some(Box::new(invoke_handler));
        self
    }

    /// Builds the App.
    pub fn build(self) -> App {
        App {
            name: self.name,
            debug: self.debug,
            invoke_handler: self.invoke_handler,
            url: self.url,
        }
    }
}

pub fn build_static(path: &Path) {}

///Builds a webview instance with all the required details.
pub fn build_webview(app: &mut App) -> Result<Webview<'static>, VeloxError> {
    let mut webview = WebviewBuilder::new()
        .debug(app.debug)
        .title(app.name)
        .width(500)
        .height(400)
        .resize(SizeHint::NONE)
        .init("")
        .dispatch(|w| {})
        .url(app.url)
        .build();

    let mut w = webview.clone();

    webview.bind("invoke", move |_seq, arg| {
        //Todo - Add logic for handling calls from javascript
        match handle_cmd(&mut w, &parse_arg(arg)) {
            Ok(()) => {}
            Err(err) => match app.run_invoke_handler(&mut w, &parse_arg(arg)) {
                Ok(handled) => {
                    // if handled {
                    //     // String::from("")
                    // } else {
                    //     // call middleware
                    // }
                }
                _ => {}
            },
        }
    });

    Ok(webview)
}

/// Parses arguments that came from javascript
pub fn parse_arg(arg: &str) -> String {
    arg.chars()
        .skip(1)
        .take(arg.chars().count() - 2)
        .collect::<String>()
}
