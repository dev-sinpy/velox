use crate::handler::call_func;
use crate::{config, events, plugin, server, Result, VeloxError};

use std::sync::{Arc, Mutex};

use wry::{
    application::{
        event_loop::{ControlFlow, EventLoop, EventLoopProxy},
        window::{Window, WindowBuilder, WindowId},
    },
    webview::{WebView, WebViewBuilder},
};

pub type InvokeHandler = Arc<
    Mutex<dyn FnMut(EventLoopProxy<events::Event>, Request) -> Option<wry::Value> + Send + Sync>,
>;

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

pub struct Application {
    pub event_loop: Option<EventLoop<events::Event>>,
    pub webviews: Vec<WindowIdendifier>,
}

pub struct WindowIdendifier {
    pub identifier: String,
    pub window_id: WindowId,
    pub webview: WebView,
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

impl Application {
    pub fn new(event_loop: Option<EventLoop<events::Event>>) -> Self {
        Self {
            event_loop,
            webviews: vec![],
        }
    }

    pub fn add_window(&mut self, window_identifier: WindowIdendifier) {
        self.webviews.push(window_identifier);
    }

    pub fn remove_window(
        &mut self,
        window_identifier: Option<String>,
        window_id: Option<WindowId>,
    ) {
        if let Some(iden) = window_identifier {
            let index = self
                .webviews
                .iter()
                .position(|item| item.identifier.contains(&iden))
                .unwrap();
            self.webviews.remove(index);
        } else if let Some(id) = window_id {
            let index = self
                .webviews
                .iter()
                .position(|item| item.window_id == id)
                .unwrap();
            self.webviews.remove(index);
        }
    }

    pub fn show_window(&mut self, window_identifier: String) {
        let index = self
            .webviews
            .iter()
            .position(|item| item.identifier.contains(&window_identifier))
            .unwrap();
        self.webviews[index].webview.window().set_visible(true);
    }

    pub fn run(mut self) {
        use wry::application::event::{Event, StartCause, WindowEvent};

        let event_loop = self.event_loop.take().unwrap();

        event_loop.run(move |event, event_loop_target, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {}
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    // self.remove_webview(None, Some(window_id));
                    if window_id == self.webviews[0].window_id {
                        *control_flow = ControlFlow::Exit;
                    }
                }

                Event::UserEvent(user_event) => {
                    use events::WindowEvents;

                    match user_event {
                        events::Event::WindowEvent(WindowEvents::AddWindow {
                            identifier,
                            window_title,
                            content,
                        }) => {
                            let splash_window = WindowBuilder::new()
                                .with_title(window_title)
                                .build(&event_loop_target)
                                .unwrap();

                            let id = splash_window.id();

                            let webview = WebViewBuilder::new(splash_window)
                                .unwrap()
                                .with_url(&content)
                                .unwrap()
                                .build()
                                .unwrap();

                            let window_identifier = WindowIdendifier {
                                identifier,
                                window_id: id,
                                webview,
                            };

                            self.add_window(window_identifier);
                        }
                        events::Event::WindowEvent(WindowEvents::ShowWindow(id)) => {
                            self.show_window(id);
                        }
                        events::Event::WindowEvent(WindowEvents::CloseWindow(id)) => {
                            self.remove_window(Some(id), None);
                        }
                        _ => {}
                    }
                }
                _ => (),
            }
        });
    }
}

impl App {
    /// Runs the app until it finishes.
    pub fn run(self) -> Result<()> {
        let application = build_webview(self)?;
        // run the webview
        application.run();

        Ok(())
    }

    /// Runs the invoke handler if defined.
    /// Returns whether the message was consumed or not.
    /// The message is considered consumed if the handler exists and returns an Ok Result.
    pub fn run_invoke_handler(
        &self,
        dispatcher: EventLoopProxy<events::Event>,
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
        F: FnMut(EventLoopProxy<events::Event>, Request) -> Option<wry::Value> + Send + Sync + 'static,
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
pub fn build_webview(app_config: App) -> Result<Application> {
    use crate::{convert_into_json, Response};
    use crossbeam_channel::unbounded;
    use wry::webview::{RpcRequest, RpcResponse};

    let (sender, receiver) = unbounded();

    let app_conf = app_config.clone();

    let event_loop = EventLoop::<events::Event>::with_user_event();

    let event_loop_proxy = event_loop.create_proxy();

    // let sender_clone = sender;

    let handler = move |_window: &Window, req: RpcRequest| {
        let params = if let wry::Value::Array(params) = req.params.unwrap() {
            params.to_vec()
        } else {
            vec![]
        };

        if let Some(id) = req.id {
            match call_func(req.method.clone(), params.clone()) {
                Ok(val) => Some(RpcResponse::new_result(Some(id), Some(val))),

                Err(err) => match err {
                    VeloxError::CommandError { detail: _ } => {
                        let request = Request::FunctionCall {
                            method_name: req.method.clone(),
                            params,
                        };

                        if let Some(value) =
                            app_config.run_invoke_handler(event_loop_proxy.clone(), request)
                        {
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
                    if let Err(err) = sender.send(event.clone()) {
                        println!("{:?}", err.to_string());
                    }

                    let request = Request::Event(event);

                    app_config.run_invoke_handler(event_loop_proxy.clone(), request);
                }

                Err(err) => {
                    println!("{:?}", err.to_string());
                }
            };
            None
        }
    };

    let window = WindowBuilder::new()
        .with_title(&app_conf.name)
        .with_visible(false)
        .build(&event_loop)
        .unwrap();

    let id = window.id();

    let webview = WebViewBuilder::new(window)?
        .with_url(&app_conf.url)?
        .with_rpc_handler(handler)
        .with_initialization_script(&init_script())
        .build()?;

    if let Some(_content) = app_conf.clone().splashscreen {
        plugin::splashscreen::show_splashscreen(event_loop.create_proxy(), app_conf, receiver)
            .unwrap();
    } else {
        webview.window().set_visible(true);
    }

    let mut app = Application::new(Some(event_loop));

    let window_identifier = WindowIdendifier {
        identifier: "main_window".to_string(),
        window_id: id,
        webview,
    };

    app.add_window(window_identifier);

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
                        window.addEventListener('load', function () {{
                          window.rpc.notify(JSON.stringify({{veloxEvent: "loaded"}}))
                        }})
                            __VELOX__.rpc = window.rpc;
                      }}
                    "#,
        velox_script = velox_script,
        test_script = test_script,
    )
}
