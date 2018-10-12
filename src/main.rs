extern crate reqwest;

#[macro_use]
extern crate serde_json;

use std::env;
use std::io::Read;
use std::process::{exit, Command, Stdio};

fn get_text<T: AsRef<str>>(prompt: T) -> Option<String> {
    let command = Command::new("rofi")
        .arg("-dmenu")
        .args(&["-lines", "0"])
        .args(&["-l", "0"])
        .args(&["-p", prompt.as_ref()])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run rofi");
    let mut result = String::new();
    command
        .stdout
        .expect("Failed to get stdout")
        .read_to_string(&mut result)
        .expect("Failed to parse output");
    result = result.replace("\n", "");
    if result.is_empty() {
        return None;
    }
    return Some(result);
}

fn show_message<T: AsRef<str>>(message: T) {
    Command::new("rofi")
        .arg("-e")
        .arg(message.as_ref())
        .spawn()
        .expect("Failed to show message");
}

fn create_task(task: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "token": env::var("TODOIST_API_TOKEN").unwrap(),
        "text": task
    });
    let response = client
        .post("https://todoist.com/api/v7/quick/add")
        .form(&payload)
        .send()
        .map_err(|e| format!("{:?}", e))?;
    return match response.error_for_status() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Got {:?} from Todoist", e.status().unwrap())),
    };
}

fn main() {
    let option = match get_text("Add Task") {
        Some(text) => text,
        None => {
            println!("No text, exiting");
            exit(1);
        }
    };

    let response_message = match create_task(option) {
        Ok(_) => String::from("Success"),
        Err(e) => format!("Error: {}", e),
    };

    show_message(response_message);
}
