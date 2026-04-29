//! Export dataset to JSON for the web chatbot.
//!
//! Usage: cargo run --bin export_dataset > docs/public/dataset.json

use nis2_chat::dataset;

fn main() {
    let entries = dataset::build_dataset();
    let mut items: Vec<serde_json::Value> = Vec::new();
    for e in &entries {
        items.push(serde_json::json!({
            "q": e.question,
            "a": e.answer,
            "c": e.category,
            "f": e.follow_ups,
        }));
    }
    println!("{}", serde_json::to_string(&items).unwrap());
}
