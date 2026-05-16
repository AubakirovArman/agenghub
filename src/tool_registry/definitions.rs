use serde_json::json;

use crate::llm_gateway::ToolDefinition;

pub fn builtin_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "read_file".to_string(),
            description:
                "Read a bounded UTF-8 text file from the current AgentHub transaction worktree."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Workspace-relative path to read"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "list_dir".to_string(),
            description:
                "List bounded directory entries inside the current AgentHub transaction worktree."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Workspace-relative directory path to list"
                    }
                },
                "required": ["path"]
            }),
        },
        ToolDefinition {
            name: "search".to_string(),
            description:
                "Search bounded UTF-8 workspace files for a literal query and return matching lines."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Literal text query to search for"
                    },
                    "path": {
                        "type": "string",
                        "description": "Optional workspace-relative file or directory to search"
                    }
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "shell".to_string(),
            description:
                "Run a short read-only shell inspection command in the current worktree. Mutating or unsafe commands are returned as approval_required instead of being executed."
                    .to_string(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "Read-only non-interactive shell command"
                    }
                },
                "required": ["command"]
            }),
        },
    ]
}
