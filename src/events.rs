//! This module includes several events that velox emits when an app is being run.

use crate::Result;

use serde::{Deserialize, Serialize};

type Identifier = String;

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
        /// Identifier for a window. For example: "main_window"
        identifier: Identifier,
        window_title: String,
        content: String,
    },
    CloseWindow(Identifier),
    ShowWindow(Identifier),
    HideWindow(Identifier),
    SetTitle {
        identifier: Identifier,
        title: String,
    },
    SetFullscreen {
        identifier: Identifier,
        flag: bool,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    VeloxEvent(VeloxEvents),
    ResourceEvent(ResourceEvent),
    NetworkEvent(NetworkEvent),
    WindowEvent(WindowEvents),
}

/// Parses event that are being sent from javascript.
pub fn parse_event(arg: &str) -> Result<Event> {
    let event: Event = serde_json::from_str(arg).unwrap();
    Ok(event)
}
