# MCP Tools

Il server MCP espone 4 tool via JSON-RPC 2.0.

## `verifica_applicabilita`

Verifica se la Direttiva NIS2 si applica a un'azienda.

### Input

| Parametro | Tipo | Obbligatorio | Descrizione |
|-----------|------|:---:|-------------|
| `settore` | `string` | Si | Codice settore NIS2 |
| `dipendenti` | `integer` | Si | Numero totale dipendenti |
| `fatturato_mln_eur` | `number` | Si | Fatturato annuo (€M) |

### Esempio

::: code-group
```json [Request]
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "verifica_applicabilita",
    "arguments": {
      "settore": "energy",
      "dipendenti": 120,
      "fatturato_mln_eur": 50.0
    }
  }
}
```

```json [Response]
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "Settore: energy\nDipendenti: 120\nFatturato: €50.0M\n\nApplicabile: SÌ\nCategoria: Essential"
    }]
  }
}
```
:::

---

## `calcola_sanzione`

Calcola la sanzione massima ai sensi dell'Art. 34 NIS2.

### Input

Stessi parametri di `verifica_applicabilita`.

### Esempio

::: code-group
```json [Request]
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "calcola_sanzione",
    "arguments": {
      "settore": "energy",
      "dipendenti": 500,
      "fatturato_mln_eur": 1000.0
    }
  }
}
```

```json [Response — Essential, 2% > floor]
{
  "jsonrpc": "2.0",
  "id": 2,
  "result": {
    "content": [{
      "type": "text",
      "text": "Categoria: Essential\nSanzione massima: €20000000\n\nBase legale: Art. 34 Direttiva (UE) 2022/2555"
    }]
  }
}
```
:::

---

## `lista_obblighi`

Elenca tutti gli obblighi NIS2 applicabili con testo legale.

### Input

Stessi parametri di `verifica_applicabilita`.

### Output

Elenco di 16 obblighi (se in ambito) con:
- Riferimento normativo (es. `Art. 21(2)(a)`)
- Descrizione
- Testo legale dalla direttiva

---

## `valuta_compliance`

Valutazione completa — restituisce il JSON `ComplianceStatus` strutturato.

### Input

| Parametro | Tipo | Obbligatorio | Descrizione |
|-----------|------|:---:|-------------|
| `nome` | `string` | Si | Ragione sociale |
| `settore` | `string` | Si | Codice settore NIS2 |
| `sotto_settore` | `string\|null` | No | Sotto-settore |
| `dipendenti` | `integer` | Si | Numero dipendenti |
| `fatturato_mln_eur` | `number` | Si | Fatturato annuo (€M) |
| `bilancio_mln_eur` | `number` | Si | Totale bilancio (€M) |
| `servizi` | `string[]` | Si | Servizi forniti |
| `stati_membri` | `string[]` | Si | Stati UE (codici ISO) |

### Output

JSON `ComplianceStatus` completo con:
- `applicable` — boolean
- `category` — Essential / Important / OutOfScope
- `obligations` — array di 16 obblighi
- `max_sanction_eur` — importo numerico
- `incident_reporting` — tempistiche 24h / 72h / 30gg

## Codici settore

### Annex I

| Codice | Settore |
|--------|---------|
| `energy` | Energia |
| `transport` | Trasporti |
| `banking` | Bancario |
| `financial_market_infrastructure` | Infrastrutture mercati finanziari |
| `health` | Sanità |
| `drinking_water` | Acque potabili |
| `waste_water` | Acque reflue |
| `digital_infrastructure` | Infrastrutture digitali |
| `ict_service_management_b2b` | Gestione servizi ICT B2B |
| `public_administration` | Pubblica amministrazione |
| `space` | Spazio |

### Annex II

| Codice | Settore |
|--------|---------|
| `postal_courier` | Servizi postali/corriere |
| `waste_management` | Gestione rifiuti |
| `chemicals` | Fabbricazione sostanze chimiche |
| `food` | Produzione/distribuzione alimentare |
| `manufacturing` | Fabbricazione |
| `digital_providers` | Fornitori di servizi digitali |
| `research` | Ricerca |
