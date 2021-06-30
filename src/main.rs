// Define modules
mod utils;

// Import modules
use utils::get_data;

// Main function
fn main() {
    let data = get_data("piyushsthr".to_string());
    // let da = serde_json::to_string(&data).unwrap();
    // let some_more_data = serde_json::json!({
    //   "query": "query",
    //   "variables": {
    //     "username": "Poiyusy"
    //   }
    // });

    let entry = data.data.media_list_collection.lists[0].entries[0];

    let entry_name = entry.media.title.english.unwrap();

    println!("Hello, world!, {}", &entry_name)
}
