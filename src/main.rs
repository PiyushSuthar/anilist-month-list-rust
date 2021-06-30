// Define modules
mod models;
mod utils;

use std::io;

// Import modules
use chrono::Datelike;
use utils::get_data;

// Main function
fn main() {
    println!("Please enter your Anilist Username:");
    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Username is required!");
    let data = get_data(username[..username.len() - 2].to_string());

    // let da = serde_json::to_string(&data).unwrap();
    // let some_more_data = serde_json::json!({
    //   "query": "query",
    //   "variables": {
    //     "username": "Poiyusy"
    //   }
    // });

    let entries = &data.data.media_list_collection.lists[0].entries;

    let mut count = 1;
    for entry in entries {
        let date = chrono::Utc::now();

        let current_month = date.month();
        let previous_month = current_month - 1;

        let entry_name = entry
            .media
            .title
            .english
            .as_ref()
            .unwrap_or(&entry.media.title.native);
        let entry_started_at = entry.started_at.month.unwrap_or(0);
        let entry_completde_at = entry.completed_at.month.unwrap_or(0);

        let is_started_this_month = entry_started_at == current_month as i64;
        let is_completed_this_month = entry_completde_at == current_month as i64;
        let is_started_previuos_month = entry_started_at == previous_month as i64;
        let is_completed_previous_month = entry_completde_at == previous_month as i64;

        if is_started_this_month
            | is_completed_previous_month
            | is_completed_this_month
            | is_started_previuos_month
        {
            println!("{}. {}", count, &entry_name);
            count += 1;
        }
    }
}
