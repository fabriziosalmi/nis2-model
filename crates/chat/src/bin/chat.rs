//! Interactive NIS2 Q&A chat with semantic cache.
//!
//! Usage:
//!   cargo run --bin chat
//!   cargo run --bin chat -- --threshold 0.85

use std::io::{self, BufRead, Write};
use std::time::Instant;

use nis2_chat::engine::ChatEngine;
use nis2_vectordb::embed::{EmbedModel, Embedder};

fn main() -> anyhow::Result<()> {
    let threshold: f32 = std::env::args()
        .position(|a| a == "--threshold")
        .and_then(|i| std::env::args().nth(i + 1))
        .and_then(|t| t.parse().ok())
        .unwrap_or(0.88);

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
    println!("Type a question (or 'quit' to exit):");
    println!("{}", "-".repeat(60));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

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
            continue;
        }

        let t = Instant::now();
        let emb = embedder.embed_one(query)?;
        let resp = engine.ask(&emb);
        let elapsed_us = t.elapsed().as_micros();

        if resp.from_cache {
            println!();
            println!("  [HIT] score={:.4}  category={}  time={}us", resp.score, resp.category, elapsed_us);
            println!("  {}", resp.answer);
        } else {
            println!();
            println!("  [MISS] No match above threshold ({:.2})  time={}us", threshold, elapsed_us);
            println!("  {}", resp.answer);
        }
        println!();
        print!("> ");
        stdout.flush()?;
    }

    println!("Bye.");
    Ok(())
}
