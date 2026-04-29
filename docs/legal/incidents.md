# Segnalazione Incidenti (Art. 23)

L'Art. 23 della Direttiva (UE) 2022/2555 stabilisce gli obblighi di segnalazione degli incidenti significativi.

## Tempistiche obbligatorie

```
Conoscenza dell'incidente
         │
         ├── entro 24 ore ──→ Preallarme (Art. 23(4)(a))
         │
         ├── entro 72 ore ──→ Notifica completa (Art. 23(4)(b))
         │
         └── entro 30 giorni → Relazione finale (Art. 23(4)(d))
```

## Dettaglio delle fasi

### Preallarme — 24 ore

**Art. 23(4)(a)**: Il soggetto deve trasmettere al CSIRT un preallarme che indichi:
- Se l'incidente è sospettato di essere causato da atti illegittimi o malevoli
- Se può avere un impatto transfrontaliero

### Notifica completa — 72 ore

**Art. 23(4)(b)**: Aggiornamento del preallarme con:
- Valutazione iniziale dell'incidente (gravità, impatto)
- Indicatori di compromissione (IoC), se disponibili

### Relazione finale — 30 giorni

**Art. 23(4)(d)**: Relazione dettagliata contenente:
- Descrizione dettagliata dell'incidente
- Tipo di minaccia o causa principale
- Misure di attenuazione applicate e in corso
- Impatto transfrontaliero, se presente

## Incidente significativo

Un incidente è considerato **significativo** (Art. 23(3)) se:

- Ha causato o può causare gravi perturbazioni operative dei servizi
- Ha causato o può causare perdite finanziarie per il soggetto
- Ha pregiudicato o può pregiudicare altre persone fisiche o giuridiche causando perdite materiali o immateriali considerevoli

## Destinatari della segnalazione

| Destinatario | Ruolo |
|-------------|-------|
| **CSIRT** | Computer Security Incident Response Team nazionale |
| **Autorità competente** | In alternativa al CSIRT, se previsto dal diritto nazionale |

## Implementazione

```rust
pub struct IncidentReporting {
    pub early_warning_hours: u32,    // 24
    pub notification_hours: u32,     // 72
    pub final_report_days: u32,      // 30
}
```

::: warning Sanzioni per mancata segnalazione
La mancata segnalazione di un incidente significativo costituisce violazione dell'Art. 23 e può dare luogo alle sanzioni previste dall'Art. 34.
:::
