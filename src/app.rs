// use crate::cmd::handle_cmd;
use crate::handler::handle_cmd;
use webview_official::{SizeHint, Webview, WebviewBuilder};
pub type InvokeHandler = Box<dyn FnMut(&mut Webview<'_>, &str) -> Result<(), String>>;

/// The application runner.
pub struct App {
    /// The JS message handler.
    pub invoke_handler: Option<InvokeHandler>,
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
    /// The JS message handler.
    invoke_handler: Option<InvokeHandler>,
    pub url: &'static str,
}

impl AppBuilder {
    /// Creates a new App builder.
    pub fn new(url: &'static str) -> Self {
        Self {
            invoke_handler: None,
            url,
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
            invoke_handler: self.invoke_handler,
            url: self.url,
        }
    }
}

pub fn build_webview(app: &mut App) -> Result<Webview<'static>, String> {
    let mut webview = WebviewBuilder::new()
        .debug(true)
        .title("Demo")
        .width(500)
        .height(400)
        .resize(SizeHint::NONE)
        .init("")
        .dispatch(|w| {})
        .url(app.url)
        .build();

    let mut w = webview.clone();

    webview.bind("invoke", move |seq, arg| {
        //Todo - Add logic for handling calls from javascript
        // w.r#return(seq, 0, "{ result: 'We always knew it!' }");

        handle_cmd(&mut w, &format_arg(arg));
        // match app.run_invoke_handler(&mut w, &arg) {
        //     Ok(handled) => {
        //         if handled {
        //             // String::from("")
        //         } else {
        //             // Err("not handled".to_string())
        //         }
        //     } // Err(e) => e,
        //     _ => {}
        // };
    });

    Ok(webview)
}

// Transform `[payload]` to `payload`
pub fn format_arg(arg: &str) -> String {
    arg.chars()
        .skip(1)
        .take(arg.chars().count() - 2)
        .collect::<String>()
}
