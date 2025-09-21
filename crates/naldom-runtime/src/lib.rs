// crates/naldom-runtime/src/lib.rs

use std::time::Duration;

// Note: In future steps, we will also define the `NaldomArray` struct here
// with `#[repr(C)]` to ensure memory layout compatibility with our C code.
// For now, we only need the sleep function.

/// This function will be called from the compiled Naldom code via C FFI.
/// It's a bridge to the Tokio runtime's sleep function.
/// The `extern "C"` makes it compatible with the C ABI.
/// `#[unsafe(no_mangle)]` ensures the function name is not changed by the compiler.
#[unsafe(no_mangle)]
pub extern "C" fn naldom_async_sleep(ms: u64) {
    // This is a simple blocking sleep for now as a placeholder.
    // In later steps, we will make this truly asynchronous by spawning
    // a Tokio task. This setup allows the compiler to generate code now,
    // and we can make the runtime implementation fully async later without
    // changing the compiled code.

    // We can add a print statement to confirm it's being called from Rust.
    // println!("Runtime (Rust): Blocking sleep for {} ms...", ms);
    std::thread::sleep(Duration::from_millis(ms));
    // println!("Runtime (Rust): Sleep finished.");
}
