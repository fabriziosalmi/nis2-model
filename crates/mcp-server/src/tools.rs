//! MCP Tool implementations — each tool wraps a deterministic rule engine function.

use serde_json::json;

use crate::protocol::{ToolCallResult, ToolDefinition};
use nis2_rules::engine;
use nis2_rules::schema::CompanyProfile;

/// Returns all tool definitions for the MCP `tools/list` response.
pub fn list_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "verifica_applicabilita".into(),
            description: "Verifica se la Direttiva NIS2 si applica a un'azienda in base a settore e dimensioni. Restituisce la classificazione (essenziale/importante/fuori ambito).".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "settore": {
                        "type": "string",
                        "description": "Codice settore NIS2 (es. 'energy', 'health', 'digital_infrastructure', 'food')"
                    },
                    "dipendenti": {
                        "type": "integer",
                        "description": "Numero totale di dipendenti"
                    },
                    "fatturato_mln_eur": {
                        "type": "number",
                        "description": "Fatturato annuo in milioni di EUR"
                    }
                },
                "required": ["settore", "dipendenti", "fatturato_mln_eur"]
            }),
        },
        ToolDefinition {
            name: "calcola_sanzione".into(),
            description: "Calcola la sanzione amministrativa pecuniaria massima prevista dall'Art. 34 NIS2 per un soggetto in ambito.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "settore": {
                        "type": "string",
                        "description": "Codice settore NIS2"
                    },
                    "dipendenti": {
                        "type": "integer",
                        "description": "Numero totale di dipendenti"
                    },
                    "fatturato_mln_eur": {
                        "type": "number",
                        "description": "Fatturato annuo in milioni di EUR"
                    }
                },
                "required": ["settore", "dipendenti", "fatturato_mln_eur"]
            }),
        },
        ToolDefinition {
            name: "lista_obblighi".into(),
            description: "Elenca tutti gli obblighi NIS2 applicabili (Art. 20, 21, 23) con il testo legale di riferimento.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "settore": {
                        "type": "string",
                        "description": "Codice settore NIS2"
                    },
                    "dipendenti": {
                        "type": "integer",
                        "description": "Numero totale di dipendenti"
                    },
                    "fatturato_mln_eur": {
                        "type": "number",
                        "description": "Fatturato annuo in milioni di EUR"
                    }
                },
                "required": ["settore", "dipendenti", "fatturato_mln_eur"]
            }),
        },
        ToolDefinition {
            name: "valuta_compliance".into(),
            description: "Esegue la valutazione completa di conformità NIS2: applicabilità, classificazione, obblighi, sanzioni e tempistiche di segnalazione incidenti.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "nome": {
                        "type": "string",
                        "description": "Ragione sociale dell'azienda"
                    },
                    "settore": {
                        "type": "string",
                        "description": "Codice settore NIS2"
                    },
                    "sotto_settore": {
                        "type": ["string", "null"],
                        "description": "Sotto-settore (opzionale)"
                    },
                    "dipendenti": {
                        "type": "integer",
                        "description": "Numero totale di dipendenti"
                    },
                    "fatturato_mln_eur": {
                        "type": "number",
                        "description": "Fatturato annuo in milioni di EUR"
                    },
                    "bilancio_mln_eur": {
                        "type": "number",
                        "description": "Totale bilancio in milioni di EUR"
                    },
                    "servizi": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Servizi forniti dall'azienda"
                    },
                    "stati_membri": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Stati membri UE in cui opera (codici ISO, es. 'IT', 'DE')"
                    }
                },
                "required": ["nome", "settore", "dipendenti", "fatturato_mln_eur", "bilancio_mln_eur", "servizi", "stati_membri"]
            }),
        },
    ]
}

/// Build a minimal CompanyProfile from the common 3-field input.
fn quick_profile(params: &serde_json::Value) -> Result<CompanyProfile, String> {
    let settore = params.get("settore")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_lowercase().replace(' ', "_"))
        .ok_or("Campo 'settore' mancante o non valido")?;
    let dipendenti = params.get("dipendenti")
        .and_then(|v| v.as_u64())
        .ok_or("Campo 'dipendenti' mancante o non valido")? as u32;
    let fatturato = params.get("fatturato_mln_eur")
        .and_then(|v| v.as_f64())
        .ok_or("Campo 'fatturato_mln_eur' mancante o non valido")?;

    Ok(CompanyProfile {
        name: "N/A".into(),
        sector: settore,
        sub_sector: None,
        employees: dipendenti,
        annual_revenue_eur_m: fatturato,
        balance_sheet_eur_m: fatturato,
        services: Vec::new(),
        member_states: vec!["IT".into()],
    })
}

/// Build a full CompanyProfile from the valuta_compliance input.
fn full_profile(params: &serde_json::Value) -> Result<CompanyProfile, String> {
    let nome = params.get("nome")
        .and_then(|v| v.as_str())
        .ok_or("Campo 'nome' mancante")?;
    let settore = params.get("settore")
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_lowercase().replace(' ', "_"))
        .ok_or("Campo 'settore' mancante")?;
    let sotto_settore = params.get("sotto_settore")
        .and_then(|v| v.as_str())
        .map(String::from);
    let dipendenti = params.get("dipendenti")
        .and_then(|v| v.as_u64())
        .ok_or("Campo 'dipendenti' mancante")? as u32;
    let fatturato = params.get("fatturato_mln_eur")
        .and_then(|v| v.as_f64())
        .ok_or("Campo 'fatturato_mln_eur' mancante")?;
    let bilancio = params.get("bilancio_mln_eur")
        .and_then(|v| v.as_f64())
        .ok_or("Campo 'bilancio_mln_eur' mancante")?;
    let servizi: Vec<String> = params.get("servizi")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();
    let stati: Vec<String> = params.get("stati_membri")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    Ok(CompanyProfile {
        name: nome.into(),
        sector: settore,
        sub_sector: sotto_settore,
        employees: dipendenti,
        annual_revenue_eur_m: fatturato,
        balance_sheet_eur_m: bilancio,
        services: servizi,
        member_states: stati,
    })
}

/// Dispatch a tool call by name.
pub fn call_tool(name: &str, params: &serde_json::Value) -> ToolCallResult {
    match name {
        "verifica_applicabilita" => tool_verifica_applicabilita(params),
        "calcola_sanzione" => tool_calcola_sanzione(params),
        "lista_obblighi" => tool_lista_obblighi(params),
        "valuta_compliance" => tool_valuta_compliance(params),
        _ => ToolCallResult::error(format!("Tool sconosciuto: {name}")),
    }
}

fn tool_verifica_applicabilita(params: &serde_json::Value) -> ToolCallResult {
    let profile = match quick_profile(params) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };
    let result = engine::evaluate(&profile);
    let text = format!(
        "Settore: {}\nDipendenti: {}\nFatturato: €{:.1}M\n\nApplicabile: {}\nCategoria: {:?}",
        profile.sector, profile.employees, profile.annual_revenue_eur_m,
        if result.applicable { "SÌ" } else { "NO" },
        result.category
    );
    ToolCallResult::text(text)
}

fn tool_calcola_sanzione(params: &serde_json::Value) -> ToolCallResult {
    let profile = match quick_profile(params) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };
    let result = engine::evaluate(&profile);
    match result.max_sanction_eur {
        Some(sanction) => {
            let text = format!(
                "Categoria: {:?}\nSanzione massima: €{:.0}\n\nBase legale: Art. 34 Direttiva (UE) 2022/2555",
                result.category, sanction
            );
            ToolCallResult::text(text)
        }
        None => ToolCallResult::text(
            "Il soggetto non rientra nell'ambito di applicazione della NIS2. Nessuna sanzione applicabile."
        ),
    }
}

fn tool_lista_obblighi(params: &serde_json::Value) -> ToolCallResult {
    let profile = match quick_profile(params) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };
    let result = engine::evaluate(&profile);
    if !result.applicable {
        return ToolCallResult::text(
            "Il soggetto non rientra nell'ambito di applicazione della NIS2. Nessun obbligo applicabile."
        );
    }

    let mut text = format!("Obblighi NIS2 applicabili ({} totali):\n\n", result.obligations.len());
    for (i, o) in result.obligations.iter().enumerate() {
        text.push_str(&format!(
            "{}. [{}] {}\n   Testo: {}\n\n",
            i + 1, o.article_ref, o.description, o.legal_text
        ));
    }
    ToolCallResult::text(text)
}

fn tool_valuta_compliance(params: &serde_json::Value) -> ToolCallResult {
    let profile = match full_profile(params) {
        Ok(p) => p,
        Err(e) => return ToolCallResult::error(e),
    };
    let result = engine::evaluate(&profile);
    match serde_json::to_string_pretty(&result) {
        Ok(json) => ToolCallResult::text(json),
        Err(e) => ToolCallResult::error(format!("Errore serializzazione: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_tools_returns_four() {
        assert_eq!(list_tools().len(), 4);
    }

    #[test]
    fn verifica_applicabilita_energy() {
        let params = json!({"settore": "energy", "dipendenti": 500, "fatturato_mln_eur": 100.0});
        let result = call_tool("verifica_applicabilita", &params);
        let text = &result.content[0].text;
        assert!(text.contains("Applicabile: SÌ"));
        assert!(text.contains("Essential"));
    }

    #[test]
    fn verifica_applicabilita_out_of_scope() {
        let params = json!({"settore": "retail", "dipendenti": 5, "fatturato_mln_eur": 0.5});
        let result = call_tool("verifica_applicabilita", &params);
        let text = &result.content[0].text;
        assert!(text.contains("Applicabile: NO"));
    }

    #[test]
    fn calcola_sanzione_essential() {
        let params = json!({"settore": "energy", "dipendenti": 500, "fatturato_mln_eur": 1000.0});
        let result = call_tool("calcola_sanzione", &params);
        let text = &result.content[0].text;
        assert!(text.contains("20000000")); // 2% of €1B
    }

    #[test]
    fn lista_obblighi_returns_16() {
        let params = json!({"settore": "energy", "dipendenti": 500, "fatturato_mln_eur": 100.0});
        let result = call_tool("lista_obblighi", &params);
        let text = &result.content[0].text;
        assert!(text.contains("16 totali"));
        assert!(text.contains("Art. 21(2)(j)"));
    }

    #[test]
    fn valuta_compliance_full() {
        let params = json!({
            "nome": "Acme Energia S.p.A.",
            "settore": "energy",
            "dipendenti": 250,
            "fatturato_mln_eur": 180.0,
            "bilancio_mln_eur": 150.0,
            "servizi": ["electricity_distribution"],
            "stati_membri": ["IT", "DE"]
        });
        let result = call_tool("valuta_compliance", &params);
        let text = &result.content[0].text;
        assert!(text.contains("\"applicable\": true"));
        assert!(text.contains("\"Essential\""));
        assert!(text.contains("early_warning_hours"));
    }

    #[test]
    fn unknown_tool_returns_error() {
        let result = call_tool("nonexistent", &json!({}));
        assert_eq!(result.is_error, Some(true));
    }

    #[test]
    fn missing_params_returns_error() {
        let result = call_tool("verifica_applicabilita", &json!({}));
        assert_eq!(result.is_error, Some(true));
    }

    #[test]
    fn verifica_applicabilita_normalization() {
        let params = json!({"settore": "  Energy ", "dipendenti": 500, "fatturato_mln_eur": 100.0});
        let result = call_tool("verifica_applicabilita", &params);
        let text = &result.content[0].text;
        assert!(text.contains("Applicabile: SÌ"));
        assert!(text.contains("Essential"));

        let params2 = json!({"settore": "digital infrastructure", "dipendenti": 10, "fatturato_mln_eur": 1.0});
        let result2 = call_tool("verifica_applicabilita", &params2);
        let text2 = &result2.content[0].text;
        assert!(text2.contains("Applicabile: SÌ"));
        assert!(text2.contains("Essential"));
    }
}
