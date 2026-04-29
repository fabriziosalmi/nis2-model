//! NIS2 Art. 21(2) obligation catalog — the 10 mandatory cybersecurity measures.
//!
//! Each obligation maps directly to a lettered sub-point of Art. 21(2).
//! This is the deterministic, immutable legal reference that the rule engine uses.

use crate::schema::{Obligation, ObligationStatus};

/// Returns all 10 obligations from NIS2 Art. 21(2)(a–j), all in `Pending` status.
pub fn art21_obligations() -> Vec<Obligation> {
    vec![
        Obligation {
            id: "nis2_art21_2_a".into(),
            article_ref: "Art. 21(2)(a)".into(),
            description: "Politiche di analisi dei rischi e di sicurezza dei sistemi informatici".into(),
            legal_text: "politiche di analisi dei rischi e di sicurezza dei sistemi informatici".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_b".into(),
            article_ref: "Art. 21(2)(b)".into(),
            description: "Gestione degli incidenti".into(),
            legal_text: "gestione degli incidenti".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_c".into(),
            article_ref: "Art. 21(2)(c)".into(),
            description: "Continuità operativa e gestione delle crisi".into(),
            legal_text: "continuità operativa, come la gestione del backup e il ripristino in caso di disastro, e gestione delle crisi".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_d".into(),
            article_ref: "Art. 21(2)(d)".into(),
            description: "Sicurezza della catena di approvvigionamento".into(),
            legal_text: "sicurezza della catena di approvvigionamento, compresi aspetti relativi alla sicurezza riguardanti i rapporti tra ciascun soggetto e i suoi diretti fornitori o fornitori di servizi".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_e".into(),
            article_ref: "Art. 21(2)(e)".into(),
            description: "Sicurezza nello sviluppo e manutenzione dei sistemi".into(),
            legal_text: "sicurezza dell'acquisizione, dello sviluppo e della manutenzione dei sistemi informatici e di rete, compresa la gestione e la divulgazione delle vulnerabilità".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_f".into(),
            article_ref: "Art. 21(2)(f)".into(),
            description: "Valutazione dell'efficacia delle misure di sicurezza".into(),
            legal_text: "strategie e procedure per valutare l'efficacia delle misure di gestione dei rischi di cibersicurezza".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_g".into(),
            article_ref: "Art. 21(2)(g)".into(),
            description: "Igiene informatica e formazione cybersecurity".into(),
            legal_text: "pratiche di igiene informatica di base e formazione in materia di cibersicurezza".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_h".into(),
            article_ref: "Art. 21(2)(h)".into(),
            description: "Politiche di crittografia e cifratura".into(),
            legal_text: "politiche e procedure relative all'uso della crittografia e, se del caso, della cifratura".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_i".into(),
            article_ref: "Art. 21(2)(i)".into(),
            description: "Sicurezza risorse umane e controllo accessi".into(),
            legal_text: "sicurezza delle risorse umane, strategie di controllo dell'accesso e gestione degli attivi".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art21_2_j".into(),
            article_ref: "Art. 21(2)(j)".into(),
            description: "Autenticazione multi-fattore e comunicazioni protette".into(),
            legal_text: "uso di soluzioni di autenticazione a più fattori o di autenticazione continua, di comunicazioni vocali, video e testuali protette e di sistemi di comunicazione di emergenza protetti all'interno del soggetto, se del caso".into(),
            status: ObligationStatus::Pending,
        },
    ]
}

/// Art. 20 NIS2 — Governance obligations.
pub fn art20_obligations() -> Vec<Obligation> {
    vec![
        Obligation {
            id: "nis2_art20_1".into(),
            article_ref: "Art. 20(1)".into(),
            description: "Approvazione misure di gestione rischi da parte degli organi direttivi".into(),
            legal_text: "Gli organi di gestione dei soggetti essenziali e importanti approvino le misure di gestione dei rischi di cibersicurezza adottate da tali soggetti per conformarsi all'articolo 21, sovraintendano alla sua attuazione e possano essere ritenuti responsabili della violazione di tale articolo da parte dei soggetti.".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art20_2".into(),
            article_ref: "Art. 20(2)".into(),
            description: "Formazione obbligatoria in cybersecurity per gli organi direttivi".into(),
            legal_text: "I membri degli organi di gestione dei soggetti essenziali e importanti siano tenuti a seguire una formazione e incoraggino i soggetti essenziali e importanti ad offrire periodicamente una formazione analoga ai loro dipendenti.".into(),
            status: ObligationStatus::Pending,
        },
    ]
}

/// Art. 23 NIS2 — Incident reporting obligations.
pub fn art23_obligations() -> Vec<Obligation> {
    vec![
        Obligation {
            id: "nis2_art23_1".into(),
            article_ref: "Art. 23(1)".into(),
            description: "Obbligo di notifica incidenti significativi al CSIRT".into(),
            legal_text: "I soggetti essenziali e importanti notifichino senza indebito ritardo al proprio CSIRT o, se del caso, alla propria autorità competente, qualsiasi incidente che abbia un impatto significativo sulla fornitura dei loro servizi.".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art23_4_a".into(),
            article_ref: "Art. 23(4)(a)".into(),
            description: "Preallarme entro 24 ore dalla conoscenza dell'incidente".into(),
            legal_text: "senza indebito ritardo, e comunque entro 24 ore da quando sono venuti a conoscenza dell'incidente significativo, un preallarme".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art23_4_b".into(),
            article_ref: "Art. 23(4)(b)".into(),
            description: "Notifica completa entro 72 ore dalla conoscenza dell'incidente".into(),
            legal_text: "senza indebito ritardo, e comunque entro 72 ore da quando sono venuti a conoscenza dell'incidente significativo, una notifica dell'incidente".into(),
            status: ObligationStatus::Pending,
        },
        Obligation {
            id: "nis2_art23_4_d".into(),
            article_ref: "Art. 23(4)(d)".into(),
            description: "Relazione finale entro 1 mese dalla notifica".into(),
            legal_text: "una relazione finale entro un mese dalla presentazione della notifica dell'incidente".into(),
            status: ObligationStatus::Pending,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn art21_has_ten_obligations() {
        assert_eq!(art21_obligations().len(), 10);
    }

    #[test]
    fn all_obligation_ids_are_unique() {
        let mut all = Vec::new();
        all.extend(art21_obligations());
        all.extend(art20_obligations());
        all.extend(art23_obligations());

        let mut ids: Vec<&str> = all.iter().map(|o| o.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "Duplicate obligation IDs found");
    }

    #[test]
    fn all_obligations_start_pending() {
        let mut all = Vec::new();
        all.extend(art21_obligations());
        all.extend(art20_obligations());
        all.extend(art23_obligations());

        for o in &all {
            assert_eq!(o.status, ObligationStatus::Pending, "Obligation {} should be Pending", o.id);
        }
    }
}
