//! Common types for ForgeIDE plugins.

use serde::{Deserialize, Serialize};

/// Context provided by ForgeIDE when invoking a plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginContext {
    /// The detected engine type for the current workspace.
    pub engine: String,
    /// Absolute path to the project root.
    pub project_path: String,
}

/// Input payload passed to the plugin entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInput {
    /// Execution context from ForgeIDE.
    pub context: PluginContext,
    /// Arbitrary JSON data from the caller (command arguments, etc.).
    #[serde(default)]
    pub data: serde_json::Value,
}

impl PluginInput {
    /// Deserialize a `PluginInput` from a raw WASM pointer and length.
    pub fn from_raw(ptr: i32, len: i32) -> Self {
        let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };
        serde_json::from_slice(bytes).unwrap_or_else(|_| PluginInput {
            context: PluginContext {
                engine: "unknown".into(),
                project_path: String::new(),
            },
            data: serde_json::Value::Null,
        })
    }
}

/// Output payload returned from the plugin entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginOutput {
    /// Human-readable message to display in the ForgeIDE UI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Arbitrary JSON data returned to the caller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Whether the plugin execution succeeded.
    #[serde(default = "default_true")]
    pub success: bool,
}

fn default_true() -> bool {
    true
}

impl PluginOutput {
    /// Create a successful output with a text message.
    pub fn text(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            data: None,
            success: true,
        }
    }

    /// Create an error output.
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            message: Some(message.into()),
            data: None,
            success: false,
        }
    }

    /// Serialize and write to WASM linear memory, returning the pointer.
    /// The caller is responsible for freeing via `dealloc`.
    pub fn into_raw(self) -> i32 {
        let json = serde_json::to_string(&self).unwrap_or_default();
        let bytes = json.as_bytes();
        let ptr = crate::alloc(bytes.len() as i32);
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), ptr as *mut u8, bytes.len());
        }
        ptr
    }
}
