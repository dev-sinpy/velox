use crate::helper::VeloxError;
use notify_rust::{Hint, Notification};

pub fn show_notification(
    summary: String,
    body: String,
    timeout: i32,
) -> Result<String, VeloxError> {
    let notification = Notification::new()
        .summary(&summary)
        .body(&body)
        // .icon("thunderbird")
        .appname("ezgui")
        // .hint(Hint::Category("email".to_owned()))
        // .hint(Hint::Resident(true))
        .timeout(timeout)
        .show()?;
    Ok("success".to_string())
}
