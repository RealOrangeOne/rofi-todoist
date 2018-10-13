extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod rofi;
mod todoist;

use std::process::exit;

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

    rofi::show_message(response_message);
}
