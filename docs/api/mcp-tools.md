# MCP Tools

The MCP server exposes 4 tools via JSON-RPC 2.0 over stdio. All tool implementations are in `crates/mcp-server/src/tools.rs`.

## verifica_applicabilita

Checks whether NIS2 applies to an entity.

**Input**: `settore` (string), `dipendenti` (integer), `fatturato_mln_eur` (number)

**Output**: Text with sector, employee count, revenue, applicability (SI/NO), and category (Essential/Important/OutOfScope).

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"verifica_applicabilita","arguments":{"settore":"energy","dipendenti":120,"fatturato_mln_eur":50.0}}}' | cargo run --bin mcp-server 2>/dev/null
```

## calcola_sanzione

Calculates the maximum Art. 34 sanction.

**Input**: same 3 fields as above.

**Output**: Category, maximum sanction in EUR, legal basis. Returns "Nessuna sanzione applicabile" for out-of-scope entities.

## lista_obblighi

Lists all applicable NIS2 obligations with legal text.

**Input**: same 3 fields.

**Output**: Numbered list of 16 obligations (if in scope) with article reference, description, and Italian legal text from the directive. Returns non-applicability message for out-of-scope entities.

## valuta_compliance

Full compliance evaluation.

**Input**: `nome` (string), `settore` (string), `sotto_settore` (string|null), `dipendenti` (integer), `fatturato_mln_eur` (number), `bilancio_mln_eur` (number), `servizi` (string[]), `stati_membri` (string[])

**Output**: Pretty-printed `ComplianceStatus` JSON with `applicable`, `category`, `obligations` (array of 16), `max_sanction_eur`, and `incident_reporting` (24h/72h/30d).
