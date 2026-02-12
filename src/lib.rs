use zed_extension_api::{
    self as zed, serde_json::Value, DebugAdapterBinary, DebugTaskDefinition,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest, Worktree,
};

struct HaskellExtension;

impl zed::Extension for HaskellExtension {
    fn new() -> Self {
        Self
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        _worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        if adapter_name == "Haskell" {
            // Ensure 'haskell-debug-adapter' is in your $PATH
            // or provide the absolute path here.

            println!("[Haskell DAP]: {:?}", config);

            Ok(DebugAdapterBinary {
                command: Some("haskell-debug-adapter".to_string()),
                arguments: vec![],
                envs: Default::default(),
                cwd: None,
                connection: Default::default(),
                request_args: StartDebuggingRequestArguments {
                    configuration: config.config,
                    request: StartDebuggingRequestArgumentsRequest::Launch,
                },
            })
        } else {
            Err(format!("Unknown adapter: {}", adapter_name))
        }
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        // println!("[haskell-debug-adapter]: {:?}", config);
        // Check the "request" field in the config to determine launch vs attach
        if let Some(request) = config.get("request").and_then(|v| v.as_str()) {
            match request {
                "launch" => Ok(StartDebuggingRequestArgumentsRequest::Launch),
                "attach" => Ok(StartDebuggingRequestArgumentsRequest::Attach),
                _ => Err(format!("Unknown request type: {}", request)),
            }
        } else {
            // Default to Launch if not specified
            Ok(StartDebuggingRequestArgumentsRequest::Launch)
        }
    }
}

zed::register_extension!(HaskellExtension);
