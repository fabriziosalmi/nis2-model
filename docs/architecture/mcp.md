# MCP Server

**Crate**: `nis2-mcp-server`

## Responsabilità

Espone il rule engine come tool richiamabili da client AI (Claude Desktop, ecc.) via [Model Context Protocol](https://modelcontextprotocol.io/) (JSON-RPC 2.0).

## Protocollo

Il server implementa la specifica MCP **2024-11-05**:

| Metodo | Descrizione |
|--------|-------------|
| `initialize` | Handshake, capabilities exchange |
| `notifications/initialized` | Conferma inizializzazione |
| `tools/list` | Elenca i 4 tool disponibili |
| `tools/call` | Esecuzione di un tool |

## Trasporto

- **stdio** (default): JSON-RPC su stdin/stdout, log su stderr

## Tool disponibili

Vedi [MCP Tools](/api/mcp-tools) per la documentazione completa.

## Architettura interna

```
stdin → JSON parse → Router → Tool dispatch → Rule Engine → JSON response → stdout
                        │
                   ┌────┼────┐
                   │    │    │
              initialize  tools/list  tools/call
                   │    │    │
                   └────┼────┘
                        │
                   stderr (log)
```

## Configurazione Claude Desktop

Copia in `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "nis2-compliance": {
      "command": "cargo",
      "args": [
        "run", "--bin", "mcp-server",
        "--manifest-path", "/path/to/nis2-model/Cargo.toml"
      ],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

## Test

```bash
cargo test -p nis2-mcp-server  # 13 test
```

Test coperti:
- Lifecycle: `initialize` → `tools/list` → `tools/call`
- Tutti i 4 tool con input validi e invalidi
- Gestione errori per metodi sconosciuti
