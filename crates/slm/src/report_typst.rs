use nis2_rules::schema::{ComplianceStatus, EntityCategory};

/// Generate a formal Typst report from compliance status.
pub fn generate_typst_report(company_name: &str, status: &ComplianceStatus) -> String {
    let mut typst = String::with_capacity(4096);

    // Document setup
    typst.push_str("#set page(paper: \"a4\", margin: (x: 2.5cm, y: 3cm))\n");
    typst.push_str("#set text(font: \"Helvetica\", size: 11pt, lang: \"it\")\n\n");

    // Header / Title Page
    typst.push_str("#align(center)[\n");
    typst.push_str("  #v(1cm)\n");
    typst.push_str("  #text(size: 24pt, weight: \"bold\", fill: rgb(\"#1a56db\"))[Report di Conformità NIS2]\n");
    typst.push_str("  #v(0.2cm)\n");
    typst.push_str(&format!("  #text(size: 14pt, style: \"italic\")[Generato per: {}]\n", company_name));
    typst.push_str("  #v(0.5cm)\n");
    typst.push_str("  #line(length: 100%, stroke: 1.5pt + rgb(\"#1a56db\"))\n");
    typst.push_str("]\n\n");

    typst.push_str("#v(0.5cm)\n\n");

    // Disclaimer
    typst.push_str("#rect(width: 100%, fill: rgb(\"#fef08a\"), inset: 12pt, radius: 4pt, stroke: 1pt + rgb(\"#eab308\"))[\n");
    typst.push_str("  #text(weight: \"bold\")[⚠️ AVVISO LEGALE:] \n");
    typst.push_str("  Questo report è generato automaticamente da un motore di analisi software.\n");
    typst.push_str("  Non costituisce consulenza legale. Per una valutazione vincolante, consultare un avvocato qualificato.\n");
    typst.push_str("]\n\n");

    // Section 1: Scope
    typst.push_str("== 1. Ambito di applicazione\n\n");
    if !status.applicable {
        typst.push_str(&format!(
            "Sulla base dei parametri forniti, il motore di analisi classifica l'azienda *{}* \
             come *non rientrante* nell'ambito di applicazione della Direttiva (UE) 2022/2555 (NIS2) \
             per i criteri di settore e dimensione valutati. Questa classificazione automatizzata \
             non esclude l'applicabilità in virtù di eccezioni previste dall'Art. 2(2) della direttiva.\n\n",
            company_name
        ));
    } else {
        let category_text = match status.category {
            EntityCategory::Essential => "Soggetto Essenziale ai sensi dell'Art. 3(1)",
            EntityCategory::Important => "Soggetto Importante ai sensi dell'Art. 3(2)",
            EntityCategory::OutOfScope => "Fuori Ambito",
        };
        typst.push_str(&format!(
            "Sulla base dei parametri forniti, il motore di analisi classifica l'azienda *{}* \
             come *rientrante* nell'ambito di applicazione della Direttiva (UE) 2022/2555 (NIS2), \
             in qualità di *{}*.\n\n",
            company_name, category_text
        ));
    }

    // Section 2: Obligations
    if status.applicable && !status.obligations.is_empty() {
        typst.push_str("== 2. Obblighi di sicurezza (Art. 21)\n\n");
        typst.push_str(&format!(
            "Risultano applicabili i seguenti *{}* obblighi di gestione dei rischi di sicurezza:\n\n",
            status.obligations.len()
        ));
        for o in &status.obligations {
            let clean_desc = o.description.replace("**", "*");
            typst.push_str(&format!("- *{}*: {}\n", o.article_ref, clean_desc));
        }
        typst.push_str("\n");
    }

    // Section 3: Sanctions
    if status.applicable {
        typst.push_str("== 3. Sanzioni potenziali (Art. 34)\n\n");
        match status.max_sanction_eur {
            Some(sanction) if sanction == 0.0 => {
                typst.push_str("Soggetto escluso dall'applicazione di sanzioni amministrative pecuniarie ai sensi delle norme di recepimento nazionali (PA in Italia).\n\n");
            }
            Some(sanction) => {
                let percentage = match status.category {
                    EntityCategory::Essential => "il 2%",
                    EntityCategory::Important => "l'1,4%",
                    _ => "",
                };
                typst.push_str(&format!(
                    "In caso di violazione degli obblighi, il soggetto è esposto a sanzioni amministrative pecuniarie fino a un massimo di *€{:.0}* (il maggiore tra il massimale fisso e {} del fatturato mondiale annuo).\n\n",
                    sanction, percentage
                ));
            }
            None => {
                typst.push_str("Nessuna sanzione applicabile.\n\n");
            }
        }
    }

    // Section 4: Incident reporting
    if status.applicable {
        if let Some(ref ir) = status.incident_reporting {
            typst.push_str("== 4. Tempistiche di segnalazione incidenti (Art. 23)\n\n");
            typst.push_str(&format!(
                "- *Preallarme* (Art. 23(4)(a)): entro *{} ore* dalla conoscenza dell'incidente.\n\
                 - *Notifica completa* (Art. 23(4)(b)): entro *{} ore* dalla conoscenza dell'incidente.\n\
                 - *Relazione finale* (Art. 23(4)(d)): entro *{} giorni* dalla notifica.\n\n",
                ir.early_warning_hours, ir.notification_hours, ir.final_report_days
            ));
        }
    }

    // Section 5: Transposition notes
    if !status.transposition_notes.is_empty() {
        typst.push_str("== 5. Note di recepimento nazionale\n\n");
        for note in &status.transposition_notes {
            typst.push_str(&format!("- {}\n", note));
        }
        typst.push_str("\n");
    }

    typst
}
