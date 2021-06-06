use wry::{application::window::WindowId, webview::WebView};

/// Describes a webview window
pub struct WebviewWindow {
    /// An identifier for a window; example: "main_window"
    pub identifier: String,
    // Window Id
    pub window_id: WindowId,
    // Instance of webview which also holds a reference to a tao window
    pub webview: WebView,
}

impl WebviewWindow {
    pub fn fullscreen(&self) {
        self.webview.window().fullscreen();
    }
    pub fn set_title(&self, title: String) {
        self.webview.window().set_title(&title);
    }
    pub fn maximize(&self, flag: bool) {
        self.webview.window().set_maximized(flag);
    }
    pub fn minimize(&self, flag: bool) {
        self.webview.window().set_minimized(flag);
    }
}
