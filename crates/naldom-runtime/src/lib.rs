// crates/naldom-runtime/src/lib.rs

use std::time::Duration;
use tokio::runtime::Runtime;

/// This is a dummy function. Its only purpose is to be called from naldom-cli
/// to create a concrete dependency, forcing Cargo to compile this crate
/// before compiling naldom-cli. This ensures the static library (.a file)
/// exists when the linker needs it.
pub fn ensure_linked() {}

// A lazy-static global tokio runtime for our compiled programs.
// This ensures we only create one runtime for the entire life of the program.
lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}

/// This function will be called from the compiled Naldom code via C FFI.
/// It is now TRULY asynchronous.
#[unsafe(no_mangle)]
pub extern "C" fn naldom_async_sleep(ms: u64) {
    // We block the C-level function call on the completion of the
    // Rust-level async task. This is the correct way to bridge
    // a synchronous world (C) to an asynchronous one (Tokio).
    TOKIO_RUNTIME.block_on(async {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    });
}
