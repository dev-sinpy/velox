use crate::app::App;
use crate::events::Event;
use crate::events::VeloxEvents;
use crate::VeloxError;

use std::thread::spawn;

use crossbeam_channel::Receiver;
use wry::{ApplicationProxy, Attributes};

/// Shows splashscreen until app finishes loading content
pub fn show_splashscreen(
    app_proxy: &ApplicationProxy,
    app_config: App,
    win_id: u32,
    receiver: Receiver<Event>,
) -> Result<(), VeloxError> {
    let app = app_proxy.clone();
    // spawn a new thread, in which create a new window and show splashcreen content
    // if recieve event of finished loading then close this window and show main window
    spawn(move || {
        let splash_window = app
            .add_window(Attributes {
                title: app_config.name,
                url: Some(app_config.splashscreen.unwrap()),
                ..Default::default()
            })
            .unwrap();

        // block this thread and receive events from javascript
        match receiver.recv().unwrap() {
            Event::VeloxEvent(velox_events) => match velox_events {
                VeloxEvents::Initialised => {}
                VeloxEvents::Loaded => {
                    // If app finished loading, then close this window and show main window
                    splash_window.hide().unwrap();
                    app.send_message(wry::Message::Window(win_id, wry::WindowMessage::Show))
                        .unwrap();
                }
            },
            Event::ResourceEvent(_event) => {}
            Event::NetworkEvent(_event) => {}
        }
    });

    Ok(())
}
