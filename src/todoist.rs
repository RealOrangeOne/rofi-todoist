use reqwest;

use serde_json::Value;
use std::env;
use std::error::Error;

static TODOIST_API_URL: &str = "https://api.todoist.com/sync/v8/quick/add";

lazy_static! {
    static ref TODOIST_API_TOKEN: String =
        env::var("TODOIST_API_TOKEN").expect("Failed to find $TODOIST_API_TOKEN");
}

pub fn format_reqwest_error(e: reqwest::Error) -> String {
    if e.is_http() {
        if let Some(url) = e.url() {
            return format!("Problem making request to {}", url);
        }
    }
    if e.is_serialization() {
        if let Some(serde_error) = e.get_ref() {
            return format!("Problem parsing response: {}", serde_error);
        }
    }
    if e.is_client_error() || e.is_server_error() {
        return format!(
            "Invalid response status: {} - {}",
            e.status().expect("Failed to get status"),
            e.description()
        );
    }
    return format!("Unknown error: {}", e.description());
}

pub fn create_task(task: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let payload = json!({
        "token": *TODOIST_API_TOKEN,
        "text": task
    });
    let mut response = client.post(TODOIST_API_URL).form(&payload).send()?;

    response = response.error_for_status()?;

    return response.json::<Value>().map(|json| {
        String::from(
            json["content"]
                .as_str()
                .expect("Response missing `content` key"),
        )
    });
}
