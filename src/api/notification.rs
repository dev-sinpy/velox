use crate::VeloxError;
use notify_rust::Notification;

pub fn show_notification(
    summary: String,
    body: String,
    timeout: i32,
) -> Result<String, VeloxError> {
    Notification::new()
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
