use crate::handler::handle_cmd;
use crate::{config, events, server, VeloxError};

use wry::{Application, Attributes, Callback, WindowProxy};

pub type InvokeHandler = Box<dyn FnMut(&mut WindowProxy, &str) -> Result<(), String> + Send>;

/// The application runner.
pub struct App {
    pub name: String,
    pub debug: bool,
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
    /// Url of the local server where frontend is hosted
    pub url: String,
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
            }
        }
    }

    /// Defines the JS message handler callback.
    pub fn invoke_handler<
        F: FnMut(&mut WindowProxy, &str) -> Result<(), String> + Send + 'static,
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
        }
    }
}

///Builds a webview instance with all the required details.
pub fn build_webview(app_config: &'static mut App) -> Result<Application, VeloxError> {
    // let mut webview = WebviewBuilder::new()
    //     .debug(app.debug)
    //     .title(app.name)
    //     .width(500)
    //     .height(400)
    //     .resize(SizeHint::NONE)
    //     .init(""
    //     //     r#"
    //     //   if (window.invoke) {{
    //     //     window.invoke(JSON.stringify({{ cmd: "__initialized" }}))
    //     //   }} else {{
    //     //     window.addEventListener('DOMContentLoaded', function () {{
    //     //       window.invoke(JSON.stringify({{ cmd: "__initialized" }}))
    //     //     }})
    //     //   }}
    //     // "#,
    //     )
    //     .dispatch(|_w| {})
    //     .url(app.url)
    //     .build();

    let webview_attrib = Attributes {
        title: app_config.name.clone(),
        url: Some(app_config.url.clone()),
        initialization_scripts: vec![],
        ..Default::default()
    };
    let callback = Callback {
        name: "invoke".to_string(),
        function: Box::new(move |mut proxy, _seq, arg| {
            // Todo - Add logic for handling calls from javascript
            match handle_cmd(&mut proxy, &parse_args(&arg)) {
                Ok(()) => {}
                Err(_err) => match events::match_events(&parse_args(&arg)) {
                    Ok(()) => {}
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
            // dispatcher
            //     .dispatch_script("console.log('The anwser is ' + window.x);")
            //     .unwrap();
            0
        }),
    };

    let mut app = Application::new()?;
    let _window1 = app.add_window(webview_attrib, Some(vec![callback]))?;
    // app.create_webview(window1, webview_attrib, Some(vec![callback]))?;

    Ok(app)
}

/// Parses arguments that came from javascript
pub fn parse_args(args: &[String]) -> String {
    // arg.chars()
    //     .skip(1)
    //     .take(arg.chars().count() - 2)
    //     .collect::<String>()
    args.first().unwrap().to_string()
}
