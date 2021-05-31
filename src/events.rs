// use crate::plugin;
use crate::VeloxError;

use serde::{Deserialize, Serialize};

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
pub enum WindowEvents {
    AddWindow {
        identifier: String,
        window_title: String,
        content: String,
    },
    ShowWindow(String),
    CloseWindow(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    VeloxEvent(VeloxEvents),
    ResourceEvent(ResourceEvent),
    NetworkEvent(NetworkEvent),
    WindowEvent(WindowEvents),
}

pub fn parse_event(arg: &str) -> Result<Event, VeloxError> {
    let event: Event = serde_json::from_str(arg).unwrap();
    Ok(event)
}
