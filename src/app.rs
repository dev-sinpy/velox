use crate::handler::call_func;
use crate::{config, events, plugin, server, VeloxError};

use std::sync::{Arc, Mutex};

use wry::{Application, Attributes, RpcRequest, WindowProxy};

pub type InvokeHandler =
    Arc<Mutex<dyn FnMut(Arc<WindowProxy>, Request) -> Option<wry::Value> + Send + Sync>>;

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

/// The application runner.
#[derive(Clone, Debug)]
pub enum Request {
    FunctionCall {
        method_name: String,
        params: Vec<wry::Value>,
    },
    Event(events::Event),
}

impl App {
    /// Runs the app until it finishes.
    pub fn run(self) -> Result<(), String> {
        let application = build_webview(self).unwrap();
        // run the webview
        application.run();

        Ok(())
    }

    /// Runs the invoke handler if defined.
    /// Returns whether the message was consumed or not.
    /// The message is considered consumed if the handler exists and returns an Ok Result.
    pub fn run_invoke_handler(
        &self,
        dispatcher: Arc<WindowProxy>,
        req: Request,
    ) -> Option<wry::Value> {
        if let Some(invoke_handler) = &self.invoke_handler {
            invoke_handler.lock().unwrap()(dispatcher, req)
        } else {
            None
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
    /// Creates a new App builder
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

    /// show splashcreen with custom html
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
        F: FnMut(Arc<WindowProxy>, Request) -> Option<wry::Value> + Send + Sync + 'static,
    >(
        mut self,
        invoke_handler: F,
    ) -> Self {
        self.invoke_handler = Some(Arc::new(Mutex::new(invoke_handler)));
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
pub fn build_webview(app_config: App) -> Result<Application, VeloxError> {
    use crate::{convert_into_json, Response};
    use crossbeam_channel::unbounded;
    use wry::webview::RpcResponse;

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

    let handler = Box::new(move |proxy: WindowProxy, req: RpcRequest| {
        let params = if let wry::Value::Array(params) = req.params.unwrap() {
            params.to_vec()
        } else {
            vec![]
        };

        if let Some(id) = req.id {
            match call_func(proxy.clone(), req.method.clone(), params.clone()) {
                Ok(val) => Some(RpcResponse::new_result(Some(id), Some(val))),

                Err(err) => match err {
                    VeloxError::CommandError { detail: _ } => {
                        let arc_proxy = Arc::new(proxy);

                        let request = Request::FunctionCall {
                            method_name: req.method.clone(),
                            params,
                        };

                        if let Some(value) = app_config.run_invoke_handler(arc_proxy, request) {
                            println!("Invoke handler found");
                            Some(RpcResponse::new_result(Some(id), Some(value)))
                        } else {
                            println!("no invoke handler");
                            None
                        }
                    }
                    _ => {
                        let res = Response::Error(err.to_string());
                        Some(RpcResponse::new_error(
                            Some(id),
                            Some(convert_into_json(res)),
                        ))
                    }
                },
            }
        } else {
            match events::parse_event(&req.method) {
                Ok(event) => {
                    if let Err(err) = s.send(event.clone()) {
                        println!("{:?}", err.to_string());
                    }
                    let arc_proxy = Arc::new(proxy);

                    let request = Request::Event(event);

                    app_config.run_invoke_handler(arc_proxy, request);
                }

                Err(err) => {
                    println!("{:?}", err.to_string());
                }
            };
            None
        }
    });

    let main_window = app.add_window_with_configs(webview_attrib, Some(handler), None, None)?;

    if let Some(_content) = app_conf.clone().splashscreen {
        main_window.hide().unwrap();

        plugin::splashscreen::show_splashscreen(&app_proxy, app_conf, main_window.id(), r)?;
    }

    Ok(app)
}

// initialise scripts to inject into javascript
fn init_script() -> String {
    let velox_script = include_str!("js/velox.js");
    let test_script = include_str!("js/velox.test.js");

    format!(
        r#"
                      {velox_script}
                    {test_script}
                      if (window.rpc) {{
                        window.rpc.notify(JSON.stringify({{veloxEvent: "initialised"}}))
                            __VELOX__.rpc = window.rpc;
                      }} else {{
                        window.addEventListener('DOMContentLoaded', function () {{
                          window.rpc.notify(JSON.stringify({{veloxEvent: "loaded"}}))
                            __VELOX__.rpc = window.rpc;
                        }})
                      }}
                    "#,
        velox_script = velox_script,
        test_script = test_script,
    )
}

// /// Parses arguments that came from javascript
// pub fn parse_args(args: &[String]) -> String {
//     args.first().unwrap().to_string()
// }
