// use crate::Result;

// use std::sync::Arc;
// use std::thread::spawn;

// use wry::{Attributes, WindowProxy};

// pub fn add_window(title: String, url: String, proxy: WindowProxy) -> Result<bool> {
//     let app_proxy = proxy.application_proxy();

//     spawn(move || {
//         app_proxy
//             .add_window(Attributes {
//                 title,
//                 url: Some(url),
//                 ..Default::default()
//             })
//             .unwrap();
//     });
//     Ok(true)
// }

// // pub fn close_window(window_id: wry::WindowId, proxy: WindowProxy) -> Result<bool> {
// //     let app_proxy = proxy.application_proxy();
// //     app_proxy.send_message(window_id)
// // }

// pub fn set_title(title: String, proxy: WindowProxy) -> Result<bool> {
//     proxy.set_title(title)?;
//     Ok(true)
// }

// pub fn maximize(proxy: Arc<WindowProxy>) -> Result<bool> {
//     proxy.maximize()?;
//     Ok(true)
// }

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

// pub fn set_fullscreen(fullscreen: bool, proxy: WindowProxy) -> Result<bool> {
//     proxy.set_fullscreen(fullscreen)?;
//     Ok(true)
// }
