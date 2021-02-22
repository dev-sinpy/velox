use crate::app::App;
use crate::events::Event;
use crate::events::VeloxEvents;
use crate::VeloxError;
use crossbeam_channel::Receiver;
use std::thread::spawn;

use wry::{ApplicationProxy, Attributes};

pub fn show_splashscreen(
    app_proxy: &ApplicationProxy,
    app_config: App,
    win_id: u32,
    receiver: Receiver<Event>,
) -> Result<(), VeloxError> {
    let app = app_proxy.clone();

    if let Some(ref _content) = app_config.splashscreen {
        spawn(move || {
            let splash_window = app
                .add_window(
                    Attributes {
                        title: app_config.name,
                        url: Some(app_config.splashscreen.unwrap()),
                        ..Default::default()
                    },
                    None,
                )
                .unwrap();

            match receiver.recv().unwrap() {
                Event::VeloxEvent(velox_events) => match velox_events {
                    VeloxEvents::Initialised => {}
                    VeloxEvents::Loaded => {
                        splash_window.hide().unwrap();
                        app.send_message(wry::Message::Window(win_id, wry::WindowMessage::Show))
                            .unwrap();
                    }
                },
                Event::ResourceEvent(_event) => {}
                Event::NetworkEvent(_event) => {}
            }
        });
    } else {
        app.send_message(wry::Message::Window(win_id, wry::WindowMessage::Show))
            .unwrap();
    }

    Ok(())
}
