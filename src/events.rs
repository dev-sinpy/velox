use std::sync::Mutex;

use crate::VeloxError;

use event_emitter_rs::EventEmitter;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

// Use lazy_static! because the size of EventEmitter is not known at compile time
lazy_static! {
    // Export the emitter with `pub` keyword
    pub static ref EVENT_EMITTER: Mutex<EventEmitter> = Mutex::new(EventEmitter::new());
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResourceEvent {
    Error,
    Abort,
    Load,
    Beforeunload,
    Unload,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum NetworkEvent {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VeloxEvents {
    Initialised,
    Loaded,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    VeloxEvent(VeloxEvents),
    ResourceEvent(ResourceEvent),
    NetworkEvent(NetworkEvent),
}

pub fn match_events(arg: &str) -> Result<(), VeloxError> {
    println!("{:?}", arg);
    let event: Event = serde_json::from_str(arg)?;
    println!("{:?}", event);
    match event {
        Event::VeloxEvent(event) => {
            println!("{:?}", event);
        }
        Event::ResourceEvent(event) => {
            println!("{:?}", event);
        }
        Event::NetworkEvent(event) => {
            println!("{:?}", event);
        }
    }
    Ok(())
}
