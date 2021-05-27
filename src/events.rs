// use crate::plugin;
use crate::VeloxError;
// use crossbeam_channel::unbounded;
use serde::{Deserialize, Serialize};
// use wry::ApplicationProxy;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ResourceEvent {
    Error,
    Abort,
    Load,
    Beforeunload,
    Unload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NetworkEvent {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum VeloxEvents {
    Initialised,
    Loaded,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    VeloxEvent(VeloxEvents),
    ResourceEvent(ResourceEvent),
    NetworkEvent(NetworkEvent),
}

pub fn parse_event(arg: &str) -> Result<Event, VeloxError> {
    let event: Event = serde_json::from_str(arg).unwrap();
    Ok(event)
}

// pub fn match_events(app_proxy: &ApplicationProxy, arg: &str) -> Result<(), VeloxError> {
//     println!("{:?}", arg);
//     let event: Event = serde_json::from_str(arg)?;
//     let (s, r) = unbounded();
//     s.send(event).unwrap();
// plugin::splashscreen::show_splashscreen(app_proxy, r).unwrap();
// let plugin = plugin::initialise_plugins();
// plugin.run_plugins(&event, app_proxy);
// // println!("{:?}", event);
// match event {
//     Event::VeloxEvent(velox_events) => {
//         EVENT_EMITTER
//             .lock()
//             .unwrap()
//             .emit("velox_event", velox_events);
//     }
//     // match velox_events {
//     //     VeloxEvents::Initialised => {
//     //         EVENT_EMITTER
//     //             .lock()
//     //             .unwrap()
//     //             .emit("velox_event", velox_events);
//     //     }
//     //     VeloxEvents::Loaded => {}
//     // },
//     Event::ResourceEvent(event) => {
//         println!("{:?}", event);
//     }
//     Event::NetworkEvent(event) => {
//         println!("{:?}", event);
//     }
// }
// Ok(())
// }
