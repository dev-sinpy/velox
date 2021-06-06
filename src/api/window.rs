//! Window API that can be used for interacting with tao window.
//! For example: changing title, changing width or height, etc.

use crate::events::{Event, WindowEvents};
use crate::Result;

use wry::application::event_loop::EventLoopProxy;

pub fn add_window(title: String, url: String, event_proxy: EventLoopProxy<Event>) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::AddWindow {
        window_title: title.clone(),
        content: url,
        identifier: title,
    }))?;
    Ok(true)
}

pub fn close_window(window_identifier: String, event_proxy: EventLoopProxy<Event>) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::CloseWindow(
        window_identifier,
    )))?;
    Ok(true)
}

pub fn set_title(
    title: String,
    window_identifier: String,
    event_proxy: EventLoopProxy<Event>,
) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::SetTitle {
        title,
        identifier: window_identifier,
    }))?;
    Ok(true)
}

pub fn maximize(
    flag: bool,
    window_identifier: String,
    event_proxy: EventLoopProxy<Event>,
) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::Maximize {
        flag,
        identifier: window_identifier,
    }))?;
    Ok(true)
}

pub fn minimize(
    flag: bool,
    window_identifier: String,
    event_proxy: EventLoopProxy<Event>,
) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::Minimize {
        flag,
        identifier: window_identifier,
    }))?;
    Ok(true)
}

// pub fn minimize(proxy: Arc<WindowProxy>) -> Result<()> {
//     proxy.minimize()?;
//     Ok(())
// }

// pub fn show(proxy: Arc<WindowProxy>) -> Result<()> {
//     proxy.show()?;
//     Ok(())
// }

// pub fn hide(proxy: Arc<WindowProxy>) -> Result<()> {
//     proxy.hide()?;
//     Ok(())
// }

// pub fn set_decorations(decorations: bool, proxy: Arc<WindowProxy>) -> Result<()> {
//     proxy.set_decorations(decorations)?;
//     Ok(())
// }

pub fn fullscreen(window_identifier: String, event_proxy: EventLoopProxy<Event>) -> Result<bool> {
    event_proxy.send_event(Event::WindowEvent(WindowEvents::SetFullscreen {
        identifier: window_identifier,
    }))?;
    Ok(true)
}
