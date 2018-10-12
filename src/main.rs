extern crate reqwest;

#[macro_use]
extern crate serde_derive;

extern crate serde;

use std::env;
use std::io::Read;
use std::process::{exit, Command, Stdio};

#[derive(Serialize, Debug)]
struct TodoistArgs {
    token: String,
    text: String,
}

impl TodoistArgs {
    pub fn from_task<T: AsRef<str>>(task: T) -> TodoistArgs {
        return TodoistArgs {
            token: env::var("TODOIST_API_TOKEN").unwrap(),
            text: task.as_ref().to_string(),
        };
    }
}

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
    let response = client
        .post("https://todoist.com/api/v7/quick/add")
        .form(&TodoistArgs::from_task(task))
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

    match create_task(option) {
        Ok(_) => show_message("Success"),
        Err(e) => show_message(format!("Error: {}", e)),
    }
}
