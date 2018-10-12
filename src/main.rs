use std::io::Read;
use std::process::{Command, Stdio};

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

fn main() {
    let option = get_text("Add Task").unwrap();
    println!("{:?}", option);

    show_message(option);
}
