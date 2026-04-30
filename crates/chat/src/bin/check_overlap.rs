use nis2_chat::dataset::build_dataset;
use std::collections::HashMap;

fn main() {
    let ds = build_dataset();
    let mut seen = HashMap::new();
    let mut duplicates = 0;
    
    for entry in ds {
        if let Some(existing_cat) = seen.insert(entry.question.clone(), entry.category.clone()) {
            println!("DUPLICATE QUESTION: '{}'\n  Cat1: {}\n  Cat2: {}\n", entry.question, existing_cat, entry.category);
            duplicates += 1;
        }
    }
    
    println!("Total duplicates: {}", duplicates);
}
