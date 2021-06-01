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
