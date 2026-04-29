# MCP Server

**Crate**: `nis2-mcp-server` | **Files**: `protocol.rs`, `tools.rs`, `server.rs` | **Tests**: 13

## What it does

Exposes the rule engine as 4 callable tools via the Model Context Protocol (MCP spec 2024-11-05). Communicates over stdin/stdout using JSON-RPC 2.0.

## Protocol methods

| Method | Description |
|--------|-------------|
| `initialize` | Handshake, returns server capabilities |
| `notifications/initialized` | Client confirms initialization |
| `tools/list` | Returns definitions for all 4 tools |
| `tools/call` | Executes a tool by name with JSON arguments |

## Tools

See [MCP Tools](/api/mcp-tools) for full input/output documentation.

- `verifica_applicabilita` -- applicability check (3 params: settore, dipendenti, fatturato_mln_eur)
- `calcola_sanzione` -- Art. 34 sanction calculation (same 3 params)
- `lista_obblighi` -- list all 16 obligations with legal text (same 3 params)
- `valuta_compliance` -- full evaluation returning ComplianceStatus JSON (8 params)

## Claude Desktop configuration

```json
{
  "mcpServers": {
    "nis2-compliance": {
      "command": "cargo",
      "args": ["run", "--bin", "mcp-server", "--manifest-path", "/path/to/nis2-model/Cargo.toml"]
    }
  }
}
```

## Tests (13)

Server tests: `initialize_returns_capabilities`, `tools_list_returns_four_tools`, `tools_call_verifica_applicabilita`, `tools_call_unknown_returns_error`, `unknown_method_returns_error`

Tool tests: `list_tools_returns_four`, `verifica_applicabilita_energy`, `verifica_applicabilita_out_of_scope`, `calcola_sanzione_essential`, `lista_obblighi_returns_16`, `valuta_compliance_full`, `unknown_tool_returns_error`, `missing_params_returns_error`
