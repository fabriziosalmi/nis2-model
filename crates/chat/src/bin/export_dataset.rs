//! Export dataset to JSON for the web chatbot.
//!
//! Restructures the flat dataset into a bilingual aligned JSON array
//! containing both English and Italian questions and answers.
//!
//! Usage: cargo run --bin export_dataset > docs/public/dataset.json

use nis2_chat::dataset;
use serde_json::json;

fn severity_for(cat: &str) -> &'static str {
    match cat {
        c if c.contains("incident") => "critical",
        c if c.contains("sanctions") => "critical",
        c if c.contains("governance") => "high",
        c if c.contains("access_control") => "high",
        c if c.contains("encryption") => "high",
        c if c.contains("business_continuity") => "high",
        c if c.contains("supply_chain") => "high",
        c if c.contains("vulnerability") => "high",
        c if c.contains("network") => "medium",
        c if c.contains("detection") => "medium",
        c if c.contains("email") => "medium",
        c if c.contains("risk") => "medium",
        c if c.contains("physical") => "medium",
        c if c.contains("development") => "medium",
        c if c.contains("asset") => "medium",
        c if c.contains("remote") => "medium",
        _ => "info",
    }
}

fn deadline_for(cat: &str) -> &'static str {
    match cat {
        c if c.contains("incident") => "24h-72h-30d",
        c if c.contains("sanctions") => "",
        c if c.contains("risk") => "annual",
        c if c.contains("business_continuity") => "annual",
        _ => "",
    }
}

fn standards_for(cat: &str) -> Vec<serde_json::Value> {
    let stubs = match cat {
        c if c.contains("incident") => vec!["ISO 27001 A.16", "NIST CSF RS.RP", "CIS 17"],
        c if c.contains("governance") => vec!["ISO 27001 A.5", "NIST CSF GV.OC"],
        c if c.contains("access_control") => vec!["ISO 27001 A.9", "NIST CSF PR.AC", "CIS 5,6"],
        c if c.contains("encryption") => vec!["ISO 27001 A.10", "NIST CSF PR.DS"],
        c if c.contains("business_continuity") => vec!["ISO 27001 A.17", "NIST CSF RC.RP"],
        c if c.contains("supply_chain") => vec!["ISO 27001 A.15", "NIST CSF ID.SC"],
        c if c.contains("vulnerability") => vec!["ISO 27001 A.12.6", "NIST CSF ID.RA", "CIS 7"],
        c if c.contains("risk") => vec!["ISO 27001 A.8", "NIST CSF ID.RA"],
        c if c.contains("network") => vec!["ISO 27001 A.13", "NIST CSF PR.AC", "CIS 12,13"],
        c if c.contains("detection") => vec!["ISO 27001 A.12.4", "NIST CSF DE.CM", "CIS 8"],
        c if c.contains("email") => vec!["ISO 27001 A.13.2", "CIS 9"],
        c if c.contains("documentation") => vec!["ISO 27001 A.7", "NIST CSF GV.PO"],
        c if c.contains("remote") => vec!["ISO 27001 A.6.2", "NIST CSF PR.AC"],
        c if c.contains("physical") => vec!["ISO 27001 A.11", "NIST CSF PR.AC"],
        c if c.contains("legal") => vec!["ISO 27001 A.18"],
        c if c.contains("sanctions") => vec!["NIS2 Art. 34"],
        c if c.contains("asset") => vec!["ISO 27001 A.8", "NIST CSF ID.AM", "CIS 1,2"],
        c if c.contains("development") => vec!["ISO 27001 A.14", "NIST CSF PR.IP", "CIS 16"],
        _ => vec![],
    };
    
    stubs.into_iter().map(|s| {
        let url = if s.starts_with("ISO") { "https://www.iso.org/isoiec-27001-information-security.html" }
        else if s.starts_with("NIST") { "https://www.nist.gov/cyberframework" }
        else if s.starts_with("CIS") { "https://www.cisecurity.org/controls/" }
        else { "https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:32022L2555" };
        serde_json::json!({ "label": s, "url": url })
    }).collect()
}

fn is_italian(text: &str) -> bool {
    let low = text.to_lowercase();
    let english_stop_words = [
        " the ", " of ", " and ", " to ", " in ", " is ", " you ", " that ", " it ", 
        " he ", " was ", " for ", " on ", " are ", " as ", " with ", " they ", " how ",
        " what ", " does ", " do ", " we ", " our ", " my ", " should ", " have ", " get "
    ];
    let padded = format!(" {} ", low.replace(|c: char| !c.is_alphabetic(), " "));
    if english_stop_words.iter().any(|&w| padded.contains(w)) {
        return false;
    }
    let markers = [
        "serve", "cosa", "come", "quali", "siamo", "nostri", "possiamo", "rientra",
        "dobbiamo", "quanto", "azienda", "cifrat", "incidente", "fornitor",
        "sicurezza", "formazione", "obblig", "sanzioni",
        "continuita", "crittografia", "rischi", "gestione", "entita", "allegato",
        "della", "delle", "nella", "nelle", "degli", "sono", "essere", "questo",
        "questa", "ogni", "deve", "devo", "fare", "perch", "anche", "ancora",
        "quando", "dove", "settori", "direttiva", "notifica", "prevede",
        "implementare", "necessario", "dipende", "operativa", "conformit",
        "misure", "procedur", "preoccup", "hackerato", "compromess",
        "violazione", "segnalazione", "adeguarsi", "rispetto", "requisiti",
        "fatturato", "dipendenti", "si ", "no "
    ];
    markers.iter().any(|&w| low.contains(w))
        || low.contains('à')
        || low.contains('è')
        || low.contains('é')
        || low.contains('ì')
        || low.contains('ò')
        || low.contains('ù')
}

fn clean_words(s: &str) -> Vec<String> {
    s.to_lowercase()
        .replace('?', "")
        .replace('.', "")
        .replace(',', "")
        .split_whitespace()
        .map(|w| w.to_string())
        .collect()
}

fn score_similarity(en_q: &str, it_q: &str) -> usize {
    let mappings = [
        ("siem", "siem"), ("soc", "soc"), ("log", "log"), ("edr", "edr"),
        ("invent", "invent"), ("classi", "classi"), ("iot", "iot"),
        ("dev", "svilupp"), ("code", "codice"), ("api", "api"),
        ("pentest", "penetration"), ("patch", "patch"), ("scan", "scans"),
        ("disclos", "disclosure"), ("bounty", "bounty"), ("priorit", "priorit"),
        ("rto", "rto"), ("backup", "backup"), ("disaster", "disaster"),
        ("continu", "continu"), ("mfa", "mfa"), ("pass", "pass"),
        ("auth", "autentic"), ("rbac", "rbac"), ("pam", "pam"),
        ("vault", "vault"), ("encryp", "cifr"), ("crypto", "critt"),
        ("tls", "tls"), ("hsm", "hsm"), ("supply", "fornit"),
        ("contract", "contrat"), ("third", "terze"), ("cloud", "cloud"),
        ("board", "cda"), ("ce", "ce"), ("incid", "incid"),
        ("hacked", "hacker"), ("ransom", "ransom"), ("phish", "phish"),
        ("train", "formaz"), ("aware", "formaz"), ("hygiene", "igiene"),
        ("gdpr", "gdpr"), ("dora", "dora"), ("compet", "compet"),
        ("acn", "acn"), ("dates", "date"), ("finan", "finan"),
        ("health", "sanit"), ("energ", "energ"), ("physic", "fisic"),
        ("audit", "audit"), ("change", "change"), ("ddos", "ddos"),
        ("mobil", "mobil"), ("smart", "cellul"), ("ot", "ot"), ("segment", "segment"),
    ];

    let en_words = clean_words(en_q);
    let it_words = clean_words(it_q);
    let mut score = 0;
    for (en_stem, it_stem) in &mappings {
        let has_en = en_words.iter().any(|w| w.contains(en_stem));
        let has_it = it_words.iter().any(|w| w.contains(it_stem));
        if has_en && has_it {
            score += 5;
        }
    }
    score
}

fn main() {
    let entries = dataset::build_dataset();

    // 1. Split entries into English, Italian, and Others
    let mut en_entries = Vec::new();
    let mut it_entries = Vec::new();

    // Pair loop questions (0..30 are EN, 30..60 are IT)
    let loop_count = 30;
    for i in 0..loop_count {
        if i < entries.len() && i + loop_count < entries.len() {
            en_entries.push(entries[i].clone());
            it_entries.push(entries[i + loop_count].clone());
        }
    }

    // Interleaved legal/ops/scenario entries:
    // scenario_entries, ops_entries, legal_entries are pushed consecutively (EN, IT, EN, IT)
    // We can identify them starting from index 150 + 24 (practical) + 20 (extended) + 54 (keywords) + deep...
    // But since it's simpler: let's filter the remaining entries (index >= 60) by language check.
    // However, some might not match exactly.
    // Let's filter everything else (index >= 60) based on `is_italian`!
    for (idx, entry) in entries.iter().enumerate().skip(60) {
        // Skip loop entries of DE, FR, ES which are between index 60 and 150
        if idx < 150 {
            continue;
        }
        if is_italian(&entry.question) {
            it_entries.push(entry.clone());
        } else {
            en_entries.push(entry.clone());
        }
    }

    // 2. Perform bipartite matching of English and Italian entries
    let mut paired_items = Vec::new();
    let mut matched_it_indices = std::collections::HashSet::new();

    // First, pair the loop questions 1-to-1 directly since we pushed them first in matching order
    for i in 0..loop_count {
        if i < en_entries.len() && i < it_entries.len() {
            paired_items.push(json!({
                "id": format!("q_{}", i),
                "c": en_entries[i].category,
                "s": severity_for(&en_entries[i].category),
                "d": deadline_for(&en_entries[i].category),
                "r": standards_for(&en_entries[i].category),
                "en": { "q": en_entries[i].question, "a": en_entries[i].answer },
                "it": { "q": it_entries[i].question, "a": it_entries[i].answer }
            }));
            matched_it_indices.insert(i);
        }
    }

    // For the remaining English entries, find the best Italian match
    let mut id_counter = loop_count;
    for (_en_idx, en_entry) in en_entries.iter().enumerate().skip(loop_count) {
        let mut best_score = 0;
        let mut best_it_idx = None;

        for (it_idx, it_entry) in it_entries.iter().enumerate().skip(loop_count) {
            if matched_it_indices.contains(&it_idx) {
                continue;
            }
            if en_entry.category != it_entry.category {
                continue;
            }
            let score = score_similarity(&en_entry.question, &it_entry.question);
            if score > best_score {
                best_score = score;
                best_it_idx = Some(it_idx);
            }
        }

        if let Some(it_idx) = best_it_idx {
            // Found a good matching category + keyword pair!
            paired_items.push(json!({
                "id": format!("q_{}", id_counter),
                "c": en_entry.category,
                "s": severity_for(&en_entry.category),
                "d": deadline_for(&en_entry.category),
                "r": standards_for(&en_entry.category),
                "en": { "q": en_entry.question, "a": en_entry.answer },
                "it": { "q": it_entries[it_idx].question, "a": it_entries[it_idx].answer }
            }));
            matched_it_indices.insert(it_idx);
        } else {
            // Fallback: search for any unmatched Italian question in the same category
            let mut fallback_it_idx = None;
            for (it_idx, it_entry) in it_entries.iter().enumerate().skip(loop_count) {
                if matched_it_indices.contains(&it_idx) {
                    continue;
                }
                if en_entry.category == it_entry.category {
                    fallback_it_idx = Some(it_idx);
                    break;
                }
            }

            if let Some(it_idx) = fallback_it_idx {
                paired_items.push(json!({
                    "id": format!("q_{}", id_counter),
                    "c": en_entry.category,
                    "s": severity_for(&en_entry.category),
                    "d": deadline_for(&en_entry.category),
                    "r": standards_for(&en_entry.category),
                    "en": { "q": en_entry.question, "a": en_entry.answer },
                    "it": { "q": it_entries[it_idx].question, "a": it_entries[it_idx].answer }
                }));
                matched_it_indices.insert(it_idx);
            } else {
                // If absolutely no Italian counterpart exists, use English Q&A as fallback for Italian
                paired_items.push(json!({
                    "id": format!("q_{}", id_counter),
                    "c": en_entry.category,
                    "s": severity_for(&en_entry.category),
                    "d": deadline_for(&en_entry.category),
                    "r": standards_for(&en_entry.category),
                    "en": { "q": en_entry.question, "a": en_entry.answer },
                    "it": { "q": en_entry.question, "a": en_entry.answer }
                }));
            }
        }
        id_counter += 1;
    }

    // Also include any remaining unmatched Italian questions as separate entries
    for (it_idx, it_entry) in it_entries.iter().enumerate().skip(loop_count) {
        if matched_it_indices.contains(&it_idx) {
            continue;
        }
        paired_items.push(json!({
            "id": format!("q_{}", id_counter),
            "c": it_entry.category,
            "s": severity_for(&it_entry.category),
            "d": deadline_for(&it_entry.category),
            "r": standards_for(&it_entry.category),
            "en": { "q": it_entry.question, "a": it_entry.answer }, // fallback to Italian since it's an Italian-only question
            "it": { "q": it_entry.question, "a": it_entry.answer }
        }));
        id_counter += 1;
    }

    println!("{}", serde_json::to_string(&paired_items).unwrap());
}
