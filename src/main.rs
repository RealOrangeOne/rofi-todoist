extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

#[cfg(feature = "notifications")]
extern crate notify_rust;

mod rofi;
mod todoist;

use std::process::exit;

#[cfg(feature = "notifications")]
pub fn show_notification<T: AsRef<str>>(message: T) {
    use notify_rust::Notification;
    Notification::new()
        .appname("todoist")
        .body(message.as_ref())
        .show()
        .expect("Failed to show notification");
}

#[cfg(not(feature = "notifications"))]
pub fn show_notification<T: AsRef<str>>(_message: T) {}

fn main() {
    let option = match rofi::get_text("Add Task") {
        Some(text) => text,
        None => {
            println!("No text, exiting");
            exit(1);
        }
    };

    let response_message = match todoist::create_task(option) {
        Ok(task_name) => format!("Created task '{}' successfully.", task_name),
        Err(e) => todoist::format_reqwest_error(e),
    };

    if cfg!(feature = "notifications") {
        // use system notification if available
        show_notification(response_message);
    } else {
        // otherwise fall back to rofi dialog
        rofi::show_message(response_message);
    }
}
