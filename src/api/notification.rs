use notify_rust::error::Error;
use notify_rust::{Hint, Notification};

pub fn show_notification(summary: String, body: String, timeout: i32) -> Result<(), Error> {
    let notification = Notification::new()
        .summary(&summary)
        .body(&body)
        // .icon("thunderbird")
        .appname("ezgui")
        // .hint(Hint::Category("email".to_owned()))
        // .hint(Hint::Resident(true))
        .timeout(timeout)
        .show()?;
    Ok(())
}
