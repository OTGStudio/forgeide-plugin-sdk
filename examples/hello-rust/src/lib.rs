use forgeide_plugin::host;

/// Plugin entry point — called by ForgeIDE with a JSON input.
/// Input: { "engine": "godot" | "unity" | "unreal", "project_path": "..." }
/// Output: { "message": "..." }
#[no_mangle]
pub extern "C" fn run(ptr: i32, len: i32) -> i32 {
    let input_bytes = unsafe {
        std::slice::from_raw_parts(ptr as *const u8, len as usize)
    };
    let input: serde_json::Value = serde_json::from_slice(input_bytes)
        .unwrap_or_default();

    let engine = input["engine"].as_str().unwrap_or("unknown");
    let message = format!("Hello from ForgeIDE plugin! Engine: {engine}");

    let output = serde_json::json!({ "message": message }).to_string();
    let output_bytes = output.as_bytes();

    let out_ptr = forgeide_plugin::alloc(output_bytes.len() as i32);
    unsafe {
        std::ptr::copy_nonoverlapping(
            output_bytes.as_ptr(),
            out_ptr as *mut u8,
            output_bytes.len(),
        );
    }
    out_ptr
}
