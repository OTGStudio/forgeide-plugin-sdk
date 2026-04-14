//! Host function bindings.
//!
//! These functions are provided by the ForgeIDE host when your plugin runs.
//! They are only available if the corresponding capability has been granted
//! by the user in the Plugin Manager.
//!
//! The extern declarations and safe wrappers are only compiled for the
//! `wasm32` target family, since the host functions are provided by the
//! ForgeIDE Wasmtime runtime.

#[cfg(target_family = "wasm")]
extern "C" {
    /// Read a file from the current project directory.
    /// Requires `filesystem:read` capability.
    ///
    /// # ABI
    /// - `path_ptr` / `path_len`: UTF-8 path relative to project root
    /// - `out_ptr` / `out_max`: caller-allocated output buffer
    /// - Returns: bytes written, or -1 on error/permission denied
    pub fn forge_read_file(path_ptr: i32, path_len: i32, out_ptr: i32, out_max: i32) -> i32;

    /// Search the ForgeIDE knowledge base.
    /// Requires `kb:read` capability.
    ///
    /// # ABI
    /// - `query_ptr` / `query_len`: UTF-8 search query
    /// - `out_ptr` / `out_max`: caller-allocated output buffer (JSON array of patterns)
    /// - Returns: bytes written, or -1 on error
    pub fn forge_kb_search(query_ptr: i32, query_len: i32, out_ptr: i32, out_max: i32) -> i32;

    /// Send a prompt through the ForgeIDE AI router.
    /// Requires `ai:routing` capability.
    /// Routes to local Ollama (respects workspace privacy policy).
    ///
    /// # ABI
    /// - `prompt_ptr` / `prompt_len`: UTF-8 prompt string
    /// - `out_ptr` / `out_max`: caller-allocated output buffer (AI response text)
    /// - Returns: bytes written, or -1 on error
    pub fn forge_ai_complete(prompt_ptr: i32, prompt_len: i32, out_ptr: i32, out_max: i32) -> i32;
}

/// Safe Rust wrapper for `forge_read_file`.
/// Requires `filesystem:read` capability.
#[cfg(target_family = "wasm")]
pub fn read_file(path: &str) -> Result<String, String> {
    let path_bytes = path.as_bytes();
    let mut out_buf = vec![0u8; 65536]; // 64KB output buffer
    let result = unsafe {
        forge_read_file(
            path_bytes.as_ptr() as i32,
            path_bytes.len() as i32,
            out_buf.as_mut_ptr() as i32,
            out_buf.len() as i32,
        )
    };
    if result < 0 {
        Err(format!("forge_read_file failed with code {result}"))
    } else {
        out_buf.truncate(result as usize);
        String::from_utf8(out_buf).map_err(|e| e.to_string())
    }
}

/// Safe Rust wrapper for `forge_kb_search`.
/// Returns a JSON string of matching patterns.
#[cfg(target_family = "wasm")]
pub fn kb_search(query: &str) -> Result<String, String> {
    let query_bytes = query.as_bytes();
    let mut out_buf = vec![0u8; 131072]; // 128KB output buffer
    let result = unsafe {
        forge_kb_search(
            query_bytes.as_ptr() as i32,
            query_bytes.len() as i32,
            out_buf.as_mut_ptr() as i32,
            out_buf.len() as i32,
        )
    };
    if result < 0 {
        Err(format!("forge_kb_search failed with code {result}"))
    } else {
        out_buf.truncate(result as usize);
        String::from_utf8(out_buf).map_err(|e| e.to_string())
    }
}

/// Stub for `read_file` on non-WASM targets (for compile-checking).
#[cfg(not(target_family = "wasm"))]
pub fn read_file(_path: &str) -> Result<String, String> {
    Err("forge_read_file is only available in the ForgeIDE WASM runtime".into())
}

/// Stub for `kb_search` on non-WASM targets (for compile-checking).
#[cfg(not(target_family = "wasm"))]
pub fn kb_search(_query: &str) -> Result<String, String> {
    Err("forge_kb_search is only available in the ForgeIDE WASM runtime".into())
}
