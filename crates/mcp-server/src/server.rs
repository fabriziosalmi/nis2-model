//! MCP Server — JSON-RPC 2.0 over stdio.
//!
//! Handles the MCP lifecycle: initialize → tools/list → tools/call.

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing::{debug, error, info};

use crate::protocol::*;
use crate::tools;

/// Run the MCP server on stdin/stdout.
pub async fn run_stdio() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let mut stdout = tokio::io::stdout();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();

    info!("NIS2 MCP Server avviato (stdio transport)");

    while let Some(line) = lines.next_line().await? {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        debug!("← {line}");

        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = JsonRpcResponse::error(None, -32700, format!("Parse error: {e}"));
                let out = serde_json::to_string(&resp)?;
                stdout.write_all(out.as_bytes()).await?;
                stdout.write_all(b"\n").await?;
                stdout.flush().await?;
                continue;
            }
        };

        let response = handle_request(request);
        let out = serde_json::to_string(&response)?;
        debug!("→ {out}");
        stdout.write_all(out.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }

    Ok(())
}

/// Route a JSON-RPC request to the appropriate handler.
fn handle_request(req: JsonRpcRequest) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => handle_initialize(req.id),
        "notifications/initialized" | "initialized" => {
            // This is a notification, no response needed — but we send one if id is present
            if req.id.is_some() {
                JsonRpcResponse::success(req.id, serde_json::json!({}))
            } else {
                // Notifications don't get responses, but we need to return something
                // for our loop — this won't be sent since id is None
                JsonRpcResponse::success(None, serde_json::json!({}))
            }
        }
        "tools/list" => handle_tools_list(req.id),
        "tools/call" => handle_tools_call(req.id, &req.params),
        method => {
            error!("Unknown method: {method}");
            JsonRpcResponse::error(req.id, METHOD_NOT_FOUND, format!("Method not found: {method}"))
        }
    }
}

fn handle_initialize(id: Option<serde_json::Value>) -> JsonRpcResponse {
    let result = InitializeResult {
        protocol_version: "2024-11-05".into(),
        capabilities: ServerCapabilities {
            tools: ToolCapabilities {
                list_changed: false,
            },
        },
        server_info: ServerInfo {
            name: "nis2-compliance-engine".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        },
    };
    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}

fn handle_tools_list(id: Option<serde_json::Value>) -> JsonRpcResponse {
    let tool_list = tools::list_tools();
    JsonRpcResponse::success(
        id,
        serde_json::json!({ "tools": tool_list }),
    )
}

fn handle_tools_call(id: Option<serde_json::Value>, params: &serde_json::Value) -> JsonRpcResponse {
    let tool_name = match params.get("name").and_then(|v| v.as_str()) {
        Some(name) => name,
        None => {
            return JsonRpcResponse::error(
                id,
                INVALID_PARAMS,
                "Missing 'name' in tools/call params",
            );
        }
    };

    let arguments = params.get("arguments").cloned().unwrap_or(serde_json::json!({}));
    let result = tools::call_tool(tool_name, &arguments);
    JsonRpcResponse::success(id, serde_json::to_value(result).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_request(method: &str, params: serde_json::Value) -> JsonRpcRequest {
        JsonRpcRequest {
            jsonrpc: "2.0".into(),
            id: Some(serde_json::json!(1)),
            method: method.into(),
            params,
        }
    }

    #[test]
    fn initialize_returns_capabilities() {
        let resp = handle_request(make_request("initialize", serde_json::json!({})));
        let result = resp.result.unwrap();
        assert_eq!(result["protocolVersion"], "2024-11-05");
        assert_eq!(result["serverInfo"]["name"], "nis2-compliance-engine");
    }

    #[test]
    fn tools_list_returns_four_tools() {
        let resp = handle_request(make_request("tools/list", serde_json::json!({})));
        let result = resp.result.unwrap();
        let tools = result["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 4);
    }

    #[test]
    fn tools_call_verifica_applicabilita() {
        let resp = handle_request(make_request("tools/call", serde_json::json!({
            "name": "verifica_applicabilita",
            "arguments": {
                "settore": "energy",
                "dipendenti": 500,
                "fatturato_mln_eur": 100.0
            }
        })));
        let result = resp.result.unwrap();
        let text = result["content"][0]["text"].as_str().unwrap();
        assert!(text.contains("Applicabile: SÌ"));
    }

    #[test]
    fn tools_call_unknown_returns_error() {
        let resp = handle_request(make_request("tools/call", serde_json::json!({
            "name": "nonexistent",
            "arguments": {}
        })));
        let result = resp.result.unwrap();
        assert_eq!(result["isError"], true);
    }

    #[test]
    fn unknown_method_returns_error() {
        let resp = handle_request(make_request("unknown/method", serde_json::json!({})));
        assert!(resp.error.is_some());
        assert_eq!(resp.error.unwrap().code, METHOD_NOT_FOUND);
    }
}
