use crate::app::App;
use crate::events::{Event, VeloxEvents};
use crate::Result;

use std::thread::spawn;

use crossbeam_channel::Receiver;
use wry::application::event_loop::EventLoopProxy;

/// Shows splashscreen until app finishes loading content
pub fn show_splashscreen(
    event_proxy: EventLoopProxy<Event>,
    app_config: App,
    receiver: Receiver<Event>,
) -> Result<()> {
    use crate::events::WindowEvents;

    event_proxy.send_event(Event::WindowEvent(WindowEvents::AddWindow {
        identifier: "splash_window".to_string(),
        window_title: app_config.name,
        content: app_config.splashscreen.unwrap(),
    }))?;

    // spawn new thread and receive event from main window
    spawn(move || loop {
        // block this thread until main window finishes loading
        if let Ok(event) = receiver.recv() {
            match event {
                Event::VeloxEvent(velox_events) => match velox_events {
                    VeloxEvents::Initialised => {}
                    VeloxEvents::Loaded => {
                        // Close splash screen window
                        event_proxy
                            .send_event(Event::WindowEvent(WindowEvents::CloseWindow(
                                "splash_window".to_string(),
                            )))
                            .unwrap();

                        // Show main window
                        event_proxy
                            .send_event(Event::WindowEvent(WindowEvents::ShowWindow(
                                "main_window".to_string(),
                            )))
                            .unwrap();
                        break;
                    }
                },
                Event::ResourceEvent(_event) => {}
                Event::NetworkEvent(_event) => {}
                _ => {}
            }
        }
    });
    Ok(())
}
