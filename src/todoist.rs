use reqwest;

use std::env;

lazy_static! {
    static ref TODOIST_API_TOKEN: String =
        env::var("TODOIST_API_TOKEN").expect("Failed to find $TODOIST_API_TOKEN");
}

pub fn create_task(task: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "token": *TODOIST_API_TOKEN,
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
