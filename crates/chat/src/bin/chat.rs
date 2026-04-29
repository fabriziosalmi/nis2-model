//! Interactive NIS2 Q&A chat with semantic cache.
//!
//! Usage:
//!   cargo run --bin chat
//!   cargo run --bin chat -- --threshold 0.80

use std::io::{self, BufRead, Write};
use std::time::Instant;

use nis2_chat::engine::ChatEngine;
use nis2_vectordb::embed::{EmbedModel, Embedder};

fn main() -> anyhow::Result<()> {
    let threshold: f32 = std::env::args()
        .position(|a| a == "--threshold")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|t| t.parse().ok())
        .unwrap_or(0.78);

    println!("nis2-model -- Interactive Q&A");
    println!("Loading embedding model (BGE-Small, 384-dim)...");

    let t0 = Instant::now();
    let embedder = Embedder::new(EmbedModel::BgeSmall)?;
    println!("Model loaded in {:.1}s", t0.elapsed().as_secs_f64());

    let mut engine = ChatEngine::new(threshold);
    let t1 = Instant::now();
    let count = engine.warmup(|text| embedder.embed_one(text))?;
    println!("Cache warmed: {} entries embedded in {:.1}s", count, t1.elapsed().as_secs_f64());
    println!("Similarity threshold: {:.2}", threshold);
    println!("Languages: EN, IT, DE, FR, ES");
    println!();
    println!("Commands: quit | stats | 1-4 (follow suggested path)");
    println!("{}", "-".repeat(60));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut last_follow_ups: Vec<String> = vec![];

    for line in stdin.lock().lines() {
        let line = line?;
        let query = line.trim();

        if query.is_empty() {
            continue;
        }
        if query == "quit" || query == "exit" || query == "q" {
            break;
        }
        if query == "stats" {
            println!("  Cache size: {} entries", engine.cache_size());
            println!("  Threshold:  {:.2}", threshold);
            println!();
            print!("> ");
            stdout.flush()?;
            continue;
        }

        // Check if user typed a number to follow a suggested path
        let effective_query = if let Ok(n) = query.parse::<usize>() {
            if n >= 1 && n <= last_follow_ups.len() {
                last_follow_ups[n - 1].clone()
            } else {
                query.to_string()
            }
        } else {
            query.to_string()
        };

        let t = Instant::now();
        let emb = embedder.embed_one(&effective_query)?;
        let resp = engine.ask(&emb);
        let elapsed_us = t.elapsed().as_micros();

        if resp.from_cache {
            println!();
            println!("  [HIT] score={:.4}  category={}  time={}us", resp.score, resp.category, elapsed_us);
            println!("  {}", resp.answer);

            // Show follow-up suggestions
            if !resp.follow_ups.is_empty() {
                println!();
                println!("  Suggested paths:");
                for (i, fu) in resp.follow_ups.iter().enumerate() {
                    println!("    [{}] {}", i + 1, fu);
                }
                last_follow_ups = resp.follow_ups;
            } else {
                last_follow_ups.clear();
            }
        } else {
            println!();
            println!("  [MISS] No match above {:.2}  time={}us", threshold, elapsed_us);
            let top3 = engine.debug_top_n(&emb, 3);
            if !top3.is_empty() {
                println!("  Nearest matches:");
                for (score, question) in &top3 {
                    println!("    {:.4}  {}", score, question);
                }
            }
            last_follow_ups.clear();
        }
        println!();
        print!("> ");
        stdout.flush()?;
    }

    println!("Bye.");
    Ok(())
}
