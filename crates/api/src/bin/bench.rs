//! Benchmark suite for the NIS2 Compliance Engine.
//!
//! Measures latency and throughput of all pipeline stages:
//! - Rule engine evaluation
//! - Report generation
//! - MCP tool dispatch
//! - API handler roundtrip
//!
//! Usage:
//!   cargo run --bin bench --release

use std::time::Instant;

use nis2_rules::engine;
use nis2_rules::schema::CompanyProfile;
use nis2_slm::inference::{InferenceBackend, TemplateBackend};
use nis2_slm::prompt::{self, InferenceParams};
use nis2_mcp_server::tools;

fn main() {
    println!("NIS2 Compliance Engine -- Benchmark Suite");
    println!("{}", "=".repeat(60));
    println!();

    let profiles = vec![
        ("Essential (energy, large)", CompanyProfile {
            name: "Energia S.p.A.".into(),
            sector: "energy".into(),
            sub_sector: None,
            employees: 500,
            annual_revenue_eur_m: 200.0,
            balance_sheet_eur_m: 180.0,
            services: vec!["electricity".into()],
            member_states: vec!["IT".into()],
        }),
        ("Important (food, medium)", CompanyProfile {
            name: "Food Corp S.r.l.".into(),
            sector: "food".into(),
            sub_sector: None,
            employees: 100,
            annual_revenue_eur_m: 20.0,
            balance_sheet_eur_m: 15.0,
            services: vec!["distribution".into()],
            member_states: vec!["IT".into()],
        }),
        ("OutOfScope (retail, micro)", CompanyProfile {
            name: "Negozio S.n.c.".into(),
            sector: "retail".into(),
            sub_sector: None,
            employees: 5,
            annual_revenue_eur_m: 0.3,
            balance_sheet_eur_m: 0.2,
            services: vec![],
            member_states: vec!["IT".into()],
        }),
    ];

    // --- Rule Engine ---
    println!("1. Rule Engine (engine::evaluate)");
    println!("{}", "-".repeat(60));
    for (label, profile) in &profiles {
        let stats = benchmark(|| { engine::evaluate(profile); }, 10_000);
        println!("   {:<35} {:>8.1} us/op  ({} ops)", label, stats.mean_us, stats.iterations);
    }
    println!();

    // --- Report Generation ---
    println!("2. Report Generation (TemplateBackend)");
    println!("{}", "-".repeat(60));
    let backend = TemplateBackend;
    let params = InferenceParams::default();
    for (label, profile) in &profiles {
        let status = engine::evaluate(profile);
        let full_prompt = prompt::build_prompt(&profile.name, &status);
        if status.applicable {
            let stats = benchmark(|| { backend.generate(&full_prompt, &params).unwrap(); }, 5_000);
            println!("   {:<35} {:>8.1} us/op  ({} ops)", label, stats.mean_us, stats.iterations);
        } else {
            println!("   {:<35}     N/A  (out of scope)", label);
        }
    }
    println!();

    // --- MCP Tool Dispatch ---
    println!("3. MCP Tool Dispatch");
    println!("{}", "-".repeat(60));
    let mcp_params = serde_json::json!({"settore": "energy", "dipendenti": 500, "fatturato_mln_eur": 200.0});
    let tool_names = ["verifica_applicabilita", "calcola_sanzione", "lista_obblighi"];
    for name in &tool_names {
        let stats = benchmark(|| { tools::call_tool(name, &mcp_params); }, 10_000);
        println!("   {:<35} {:>8.1} us/op  ({} ops)", name, stats.mean_us, stats.iterations);
    }

    let full_params = serde_json::json!({
        "nome": "Bench S.p.A.", "settore": "energy", "dipendenti": 500,
        "fatturato_mln_eur": 200.0, "bilancio_mln_eur": 180.0,
        "servizi": ["electricity"], "stati_membri": ["IT"]
    });
    let stats = benchmark(|| { tools::call_tool("valuta_compliance", &full_params); }, 5_000);
    println!("   {:<35} {:>8.1} us/op  ({} ops)", "valuta_compliance", stats.mean_us, stats.iterations);
    println!();

    // --- JSON Serialization ---
    println!("4. JSON Serialization (ComplianceStatus)");
    println!("{}", "-".repeat(60));
    let status = engine::evaluate(&profiles[0].1);
    let stats = benchmark(|| { serde_json::to_string(&status).unwrap(); }, 50_000);
    println!("   {:<35} {:>8.1} us/op  ({} ops)", "to_string (essential)", stats.mean_us, stats.iterations);
    let json_str = serde_json::to_string(&status).unwrap();
    let stats = benchmark(|| { serde_json::from_str::<nis2_rules::schema::ComplianceStatus>(&json_str).unwrap(); }, 50_000);
    println!("   {:<35} {:>8.1} us/op  ({} ops)", "from_str (essential)", stats.mean_us, stats.iterations);
    println!();

    // --- Summary ---
    println!("{}", "=".repeat(60));
    println!("All benchmarks complete.");
}

struct BenchStats {
    mean_us: f64,
    iterations: u64,
}

fn benchmark<F: Fn()>(f: F, iterations: u64) -> BenchStats {
    // Warmup
    for _ in 0..10 {
        f();
    }

    let start = Instant::now();
    for _ in 0..iterations {
        f();
    }
    let elapsed = start.elapsed();

    BenchStats {
        mean_us: elapsed.as_secs_f64() * 1_000_000.0 / iterations as f64,
        iterations,
    }
}
