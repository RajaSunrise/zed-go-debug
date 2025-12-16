use std::fs;
use std::path::Path;
use zed_extension_api::{
    self as zed, serde_json, DebugAdapterBinary, DebugTaskDefinition, Extension, Result,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

struct GoDebuggerPro {
    cached_binary_path: Option<String>,
}

impl GoDebuggerPro {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn find_delve_binary(&mut self, worktree: &Worktree) -> Result<String, String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                return Ok(path.clone());
            }
        }

        if let Some(path) = worktree.which("dlv") {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let home = std::env::var("HOME").map_err(|e| format!("Could not get HOME: {}", e))?;
        let go_bin = Path::new(&home).join("go").join("bin").join("dlv");
        if go_bin.exists() {
            let path = go_bin.to_string_lossy().to_string();
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let common_paths = [
            "/usr/local/bin/dlv",
            "/opt/homebrew/bin/dlv",
            "/usr/bin/dlv",
        ];

        for p in common_paths {
            if fs::metadata(p).is_ok() {
                let path = p.to_string();
                self.cached_binary_path = Some(path.clone());
                return Ok(path);
            }
        }

        Err("Could not find 'dlv' binary. Please install Delve (go install github.com/go-delve/delve/cmd/dlv@latest) and ensure it is in your PATH.".to_string())
    }
}

impl Extension for GoDebuggerPro {
    fn new() -> Self {
        Self::new()
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        if adapter_name != "go-delve-pro" {
            return Err(format!("Unknown adapter: {}", adapter_name));
        }

        let path = if let Some(path) = user_provided_debug_adapter_path {
            path
        } else {
            self.find_delve_binary(worktree)?
        };

        // config.config is the JSON string
        let config_val: serde_json::Value = serde_json::from_str(&config.config)
            .map_err(|e| format!("Failed to parse config JSON: {}", e))?;

        let request_type = if config_val.get("request").and_then(|v| v.as_str()) == Some("attach") {
            StartDebuggingRequestArgumentsRequest::Attach
        } else {
            StartDebuggingRequestArgumentsRequest::Launch
        };

        Ok(DebugAdapterBinary {
            command: Some(path),
            arguments: vec!["dap".to_string()],
            envs: vec![],
            cwd: None,
            connection: None,
            request_args: StartDebuggingRequestArguments {
                configuration: config.config,
                request: request_type,
            },
        })
    }

    fn suggest_docs_packages(&self, provider: String) -> Result<Vec<String>, String> {
        if provider == "go" {
            Ok(vec![
                "fmt".to_string(),
                "os".to_string(),
                "net/http".to_string(),
                "encoding/json".to_string(),
                "time".to_string(),
                "context".to_string(),
            ])
        } else {
            Ok(vec![])
        }
    }
}

zed::register_extension!(GoDebuggerPro);
