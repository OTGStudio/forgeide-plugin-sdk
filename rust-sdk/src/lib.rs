//! ForgeIDE Plugin SDK
//!
//! Build ForgeIDE plugins in Rust. Plugins compile to WebAssembly and run in
//! ForgeIDE's Wasmtime sandbox with capability-gated host functions.
//!
//! # Quick start
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! forgeide-plugin = "0.1"
//!
//! [lib]
//! crate-type = ["cdylib"]
//! ```
//!
//! ```rust,no_run
//! use forgeide_plugin::prelude::*;
//!
//! #[no_mangle]
//! pub extern "C" fn run(ptr: i32, len: i32) -> i32 {
//!     let input: PluginInput = PluginInput::from_raw(ptr, len);
//!     let output = PluginOutput::text(format!("Hello from {}!", input.context.engine));
//!     output.into_raw()
//! }
//! ```

pub mod prelude;
pub mod host;
pub mod types;

// Memory management exports — required by ForgeIDE host
// These are called by the host to allocate/free memory for JSON payloads.

/// Allocate `len` bytes in WASM linear memory. Returns pointer.
/// Required export: ForgeIDE calls this before writing JSON input.
#[no_mangle]
pub extern "C" fn alloc(len: i32) -> i32 {
    let mut buf: Vec<u8> = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr() as i32;
    std::mem::forget(buf);
    ptr
}

/// Free `len` bytes at `ptr`. Called by host after reading output.
#[no_mangle]
pub extern "C" fn dealloc(ptr: i32, len: i32) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
    }
}

/// Capability string constants — use these in your forgeide-plugin.json manifest.
pub mod capabilities {
    pub const FILESYSTEM_READ: &str = "filesystem:read";
    pub const FILESYSTEM_WRITE: &str = "filesystem:write";
    pub const NETWORK_LOCAL: &str = "network:local";
    pub const NETWORK_INTERNET: &str = "network:internet";
    pub const PROCESS_SPAWN: &str = "process:spawn";
    pub const KB_READ: &str = "kb:read";
    pub const KB_WRITE: &str = "kb:write";
    pub const AI_ROUTING: &str = "ai:routing";
}
