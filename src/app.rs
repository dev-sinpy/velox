use crate::handler::handle_cmd;
use crate::{config, events, plugin, server, VeloxError};

use wry::{Application, Attributes, Callback, WindowProxy};

use dyn_clonable::*;

#[clonable]
pub trait NewTrait: FnMut(&mut WindowProxy, &str) -> Result<(), String> + Clone {}

pub type InvokeHandler = Box<dyn NewTrait + Send + Sync>;

pub enum ContentType {
    Url(String),
    Html(String),
}

/// The application runner.
#[derive(Clone)]
pub struct App {
    pub name: String,
    pub debug: bool,
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
    /// Url of the local server where frontend is hosted
    pub url: String,
    pub splashscreen: Option<String>,
}

impl App {
    /// Runs the app until it finishes.
    pub fn run(self) -> Result<(), String> {
        let application = build_webview(Box::leak(Box::new(self))).unwrap();
        // run the webview
        application.run();

        Ok(())
    }

    /// Runs the invoke handler if defined.
    /// Returns whether the message was consumed or not.
    /// The message is considered consumed if the handler exists and returns an Ok Result.
    pub fn run_invoke_handler(
        &mut self,
        dispatcher: &mut WindowProxy,
        arg: &str,
    ) -> Result<bool, String> {
        if let Some(ref mut invoke_handler) = self.invoke_handler {
            invoke_handler(dispatcher, arg).map(|_| true)
        } else {
            Ok(false)
        }
    }
}

/// The App builder.
pub struct AppBuilder {
    pub name: String,
    pub debug: bool,
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
    /// Url of the local server where frontend is hosted
    pub url: String,
    pub splashscreen: Option<String>,
}

impl AppBuilder {
    /// Creates a new App builder.

    pub fn from_config(config: String) -> Self {
        use portpicker::pick_unused_port;

        let config = config::parse_config(&config).unwrap();
        let arg = std::env::args().find(|arg| arg.contains("target"));

        if let Some(_arg) = arg {
            Self {
                name: config.name,
                debug: config.debug,
                invoke_handler: None,
                url: config.dev_server_url,
                splashscreen: None,
            }
        } else {
            let port = pick_unused_port().expect("no unused port");
            let url = format!("127.0.0.1:{}", port);
            server::spawn_server(&url, config.clone());
            Self {
                name: config.name,
                debug: config.debug,
                invoke_handler: None,
                url: "http://".to_owned() + &url,
                splashscreen: None,
            }
        }
    }

    pub fn show_splashscreen(self, content: String) -> Self {
        Self {
            name: self.name,
            debug: self.debug,
            invoke_handler: self.invoke_handler,
            url: self.url,
            splashscreen: Some("data:text/html,".to_string() + &content),
        }
    }

    /// Defines the JS message handler callback.
    pub fn invoke_handler<
        F: FnMut(&mut WindowProxy, &str) -> Result<(), String> + Send + Sync + 'static + NewTrait,
    >(
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
            splashscreen: self.splashscreen,
        }
    }
}

///Builds a webview instance with all the required details.
pub fn build_webview(app_config: &'static mut App) -> Result<Application, VeloxError> {
    use crossbeam_channel::unbounded;

    let mut app = Application::new()?;

    let app_proxy = app.application_proxy();

    let webview_attrib = Attributes {
        title: app_config.name.clone(),
        url: Some(app_config.url.clone()),
        initialization_scripts: vec![init_script()],
        ..Default::default()
    };

    let (s, r) = unbounded();

    let app_conf = app_config.clone();

    let callback = Callback {
        name: "invoke".to_string(),
        function: Box::new(move |mut proxy, _seq, arg| {
            // Todo - Add logic for handling calls from javascript
            match handle_cmd(&mut proxy, &parse_args(&arg)) {
                Ok(()) => {}
                // Err(_err) => match events::match_events(&app_proxy, &parse_args(&arg)) {
                Err(_err) => match events::parse_event(&parse_args(&arg)) {
                    Ok(event) => {
                        if let Err(err) = s.send(event) {
                            println!("{:?}", err.to_string());
                        }
                    }
                    Err(err) => {
                        println!("{:?}", err.to_string());
                        if let Ok(handled) =
                            app_config.run_invoke_handler(&mut proxy, &parse_args(&arg))
                        {
                            if handled {
                                // String::from("")
                                println!("handled");
                            } else {
                                println!("not handled");
                            }
                        }
                    }
                },
            }
            0
        }),
    };

    let main_window = app.add_window(webview_attrib, Some(vec![callback]))?;

    main_window.hide().unwrap();
    plugin::splashscreen::show_splashscreen(&app_proxy, app_conf, main_window.id(), r)?;

    Ok(app)
}

fn init_script() -> String {
    let velox_script = include_str!("js/velox.js");
    format!(
        r#"
                      {velox_script}
                      if (window.invoke) {{
                        window.invoke(JSON.stringify({{veloxEvent: "initialised"}}))
                      }} else {{
                        window.addEventListener('DOMContentLoaded', function () {{
                          window.invoke(JSON.stringify({{veloxEvent: "loaded"}}))
                        }})
                      }}
                    "#,
        velox_script = velox_script
    )
}

/// Parses arguments that came from javascript
pub fn parse_args(args: &[String]) -> String {
    args.first().unwrap().to_string()
}
