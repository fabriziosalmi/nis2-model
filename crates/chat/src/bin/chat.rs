//! Interactive NIS2 Q&A chat with semantic cache and session tracking.
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
    println!();
    println!("Commands: quit | stats | reset | coverage | 1-6 (follow path)");
    println!("{}", "-".repeat(60));

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut last_follow_ups: Vec<String> = vec![];

    for line in stdin.lock().lines() {
        let line = line?;
        let query = line.trim();

        if query.is_empty() { continue; }
        if query == "quit" || query == "exit" || query == "q" { break; }

        if query == "stats" {
            let cov = engine.session.coverage();
            println!("  Cache:     {} entries", engine.cache_size());
            println!("  Threshold: {:.2}", threshold);
            println!("  Visited:   {} questions", engine.session.questions_visited());
            println!("  Coverage:  {}/{} NIS2 areas", cov.explored, cov.total);
            println!();
            print!("> "); stdout.flush()?;
            continue;
        }

        if query == "reset" {
            engine.session.reset();
            last_follow_ups.clear();
            println!("  Session reset. Coverage: 0/16");
            println!();
            print!("> "); stdout.flush()?;
            continue;
        }

        if query == "coverage" {
            let cov = engine.session.coverage();
            println!();
            println!("  NIS2 Compliance Coverage: {}/{}", cov.explored, cov.total);
            print!("  [");
            for i in 0..cov.total {
                if i < cov.explored { print!("#"); } else { print!("."); }
            }
            println!("]");
            if !cov.missing.is_empty() {
                println!();
                println!("  Unexplored areas:");
                for (i, area) in cov.missing.iter().enumerate() {
                    println!("    {} {}", i + 1, area);
                }
            } else {
                println!("  All 16 obligation areas explored!");
            }
            println!();
            print!("> "); stdout.flush()?;
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
            println!("  [HIT] score={:.4}  category={}  time={}us  coverage={}/{}",
                resp.score, resp.category, elapsed_us,
                resp.coverage.explored, resp.coverage.total);
            println!("  {}", resp.answer);

            if !resp.follow_ups.is_empty() {
                println!();
                println!("  Next:");
                for (i, fu) in resp.follow_ups.iter().enumerate() {
                    println!("    [{}] {}", i + 1, fu);
                }
                last_follow_ups = resp.follow_ups;
            } else {
                println!();
                println!("  All areas explored! Type 'coverage' to see status.");
                last_follow_ups.clear();
            }
        } else {
            println!();
            println!("  [MISS] No match above {:.2}  time={}us", threshold, elapsed_us);
            let top3 = engine.debug_top_n(&emb, 3);
            if !top3.is_empty() {
                println!("  Nearest:");
                for (score, question) in &top3 {
                    println!("    {:.4}  {}", score, question);
                }
            }
            if !resp.follow_ups.is_empty() {
                println!();
                println!("  Try these instead:");
                for (i, fu) in resp.follow_ups.iter().enumerate() {
                    println!("    [{}] {}", i + 1, fu);
                }
                last_follow_ups = resp.follow_ups;
            }
        }
        println!();
        print!("> ");
        stdout.flush()?;
    }

    // Final coverage on exit
    let cov = engine.session.coverage();
    println!();
    println!("Session: {} questions, {}/{} areas covered.", 
        engine.session.questions_visited(), cov.explored, cov.total);
    println!("Bye.");
    Ok(())
}
