use crate::Result;

use std::sync::Arc;

use wry::{Attributes, WindowProxy};

// pub fn add_window(title: String, url: String, proxy: Arc<WindowProxy>) -> Result<()> {
//     proxy
//         .add_window(
//             Attributes {
//                 title,
//                 url: Some(url),
//                 ..Default::default()
//             },
//             None,
//         )
//         .unwrap();
//     Ok(())
// }

pub fn set_title(title: String, proxy: WindowProxy) -> Result<bool> {
    proxy.set_title(title)?;
    Ok(true)
}

pub fn maximize(proxy: Arc<WindowProxy>) -> Result<bool> {
    proxy.maximize()?;
    Ok(true)
}

pub fn minimize(proxy: Arc<WindowProxy>) -> Result<()> {
    proxy.minimize()?;
    Ok(())
}

pub fn show(proxy: Arc<WindowProxy>) -> Result<()> {
    proxy.show()?;
    Ok(())
}

pub fn hide(proxy: Arc<WindowProxy>) -> Result<()> {
    proxy.hide()?;
    Ok(())
}

pub fn set_decorations(decorations: bool, proxy: Arc<WindowProxy>) -> Result<()> {
    proxy.set_decorations(decorations)?;
    Ok(())
}

pub fn set_fullscreen(fullscreen: bool, proxy: WindowProxy) -> Result<bool> {
    proxy.set_fullscreen(fullscreen)?;
    Ok(true)
}

pub fn open(url: String) {}
