//! Count unique conversation paths through the Italian entries.
//!
//! Usage: cargo run --bin paths

use nis2_chat::cache::CacheEntry;
use nis2_chat::dataset;

fn main() {
    let entries = dataset::build_dataset();

    // Italian entry points: questions containing Italian markers
    let it_markers = [
        "si applica", "settore", "soggetto", "sanzioni", "obblighi",
        "scadenze", "azienda", "autenticazione", "catena", "crittografia",
        "formazione", "governance", "soglia", "differenza", "DORA",
        "rotazione", "condivise", "chiavi ssh", "multifattore",
        "cifratura", "gestione segreti", "notifica incidente",
        "24 ore", "72 ore", "risposta incidenti", "continuita",
        "approvvigionamento", "fornitori", "responsabilita",
        "analisi rischi", "segmentazione", "multa",
    ];

    let italian_entries: Vec<&CacheEntry> = entries.iter()
        .filter(|e| {
            let q = e.question.to_lowercase();
            it_markers.iter().any(|m| q.contains(&m.to_lowercase()))
        })
        .collect();

    println!("=== NIS2 Chat Path Analysis (Italian) ===");
    println!();
    println!("Total entries in dataset: {}", entries.len());
    println!("Italian entry points:     {}", italian_entries.len());
    println!();

    // Build a lookup: question -> entry
    let all_questions: std::collections::HashMap<String, &CacheEntry> = entries.iter()
        .map(|e| (e.question.clone(), e))
        .collect();

    // Count paths at depth 1, 2, 3
    let mut paths_1 = 0usize;
    let mut paths_2 = 0usize;
    let mut paths_3 = 0usize;
    let mut unique_paths: Vec<Vec<String>> = vec![];

    for entry in &italian_entries {
        // Depth 1: just the entry itself
        paths_1 += 1;

        // Depth 2: entry -> each follow-up
        for fu1 in &entry.follow_ups {
            paths_2 += 1;

            // Depth 3: entry -> follow-up -> follow-up's follow-ups
            if let Some(fu1_entry) = all_questions.get(fu1) {
                for fu2 in &fu1_entry.follow_ups {
                    // Avoid cycles
                    if fu2 != &entry.question && fu2 != fu1 {
                        paths_3 += 1;
                        unique_paths.push(vec![
                            entry.question.clone(),
                            fu1.clone(),
                            fu2.clone(),
                        ]);
                    }
                }
            }
        }
    }

    println!("Paths at depth 1 (entry points):  {}", paths_1);
    println!("Paths at depth 2 (1 follow-up):   {}", paths_2);
    println!("Paths at depth 3 (2 follow-ups):  {}", paths_3);
    println!("Total unique paths (all depths):  {}", paths_1 + paths_2 + paths_3);
    println!();

    // Show some example paths
    println!("--- Example 3-step paths ---");
    let mut shown = std::collections::HashSet::new();
    for path in unique_paths.iter().take(20) {
        let key = format!("{} > {} > {}", path[0], path[1], path[2]);
        if shown.insert(key.clone()) {
            println!("  {} ", path[0]);
            println!("    -> {}", path[1]);
            println!("      -> {}", path[2]);
            println!();
        }
    }

    // Category distribution
    println!("--- Italian entries by category ---");
    let mut cat_count: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for e in &italian_entries {
        *cat_count.entry(e.category.clone()).or_default() += 1;
    }
    let mut cats: Vec<_> = cat_count.into_iter().collect();
    cats.sort_by(|a, b| b.1.cmp(&a.1));
    for (cat, count) in &cats {
        println!("  {:30} {}", cat, count);
    }
}
