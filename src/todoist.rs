use reqwest;

use serde_json::Value;
use std::env;

static TODOIST_API_URL: &str = "https://todoist.com/api/v7/quick/add";

lazy_static! {
    static ref TODOIST_API_TOKEN: String =
        env::var("TODOIST_API_TOKEN").expect("Failed to find $TODOIST_API_TOKEN");
}

pub fn create_task(task: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "token": *TODOIST_API_TOKEN,
        "text": task
    });
    let mut response = client
        .post(TODOIST_API_URL)
        .form(&payload)
        .send()
        .map_err(|e| format!("{:?}", e))?;

    let status_code = response.status();

    if !status_code.is_success() {
        return Err(format!("Got {} from Todoist", status_code));
    }

    return match response.json::<Value>() {
        Ok(json) => Ok(String::from(
            json["content"]
                .as_str()
                .expect("Response missing `content` key"),
        )),
        Err(_) => unreachable!(),
    };
}
