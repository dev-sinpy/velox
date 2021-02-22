pub mod splashscreen;

use crate::events::Event;
use crate::VeloxError;
use wry::ApplicationProxy;

type PluginFunc = Box<dyn FnOnce(&Event, &ApplicationProxy) -> Result<(), VeloxError>>;

pub struct Plugin {
    plugins: Vec<PluginFunc>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin {
    pub fn new() -> Self {
        Self { plugins: vec![] }
    }

    pub fn add_plugin(&mut self, plugin: PluginFunc) {
        self.plugins.push(plugin);
    }
    pub fn run_plugins(self, event: &Event, app_proxy: &ApplicationProxy) {
        for plugin in self.plugins {
            plugin(&event, app_proxy).unwrap();
        }
    }
}

pub fn initialise_plugins() -> Plugin {
    let mut plugin = Plugin::new();
    // plugin.add_plugin(Box::new(|event, app_proxy| {
    //     // splashscreen::show_splashscreen(event, app_proxy)
    // }));
    plugin
}
