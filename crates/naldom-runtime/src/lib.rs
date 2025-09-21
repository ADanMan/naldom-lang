// crates/naldom-runtime/src/lib.rs

use std::time::Duration;
use tokio::runtime::Runtime;

lazy_static::lazy_static! {
    static ref TOKIO_RUNTIME: Runtime = Runtime::new().expect("Failed to create Tokio runtime");
}

/// A dummy function to force Cargo to link this crate.
pub fn ensure_linked() {}

/// The FFI function called from compiled Naldom code.
#[unsafe(no_mangle)]
pub extern "C" fn naldom_async_sleep(ms: u64) {
    TOKIO_RUNTIME.block_on(async {
        tokio::time::sleep(Duration::from_millis(ms)).await;
    });
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant; // FIX: Import the Instant struct

    #[test]
    fn test_naldom_async_sleep_blocks_for_duration() {
        // Arrange
        let sleep_duration_ms = 100;
        let start = Instant::now();

        // Act
        naldom_async_sleep(sleep_duration_ms);
        let elapsed = start.elapsed();

        // Assert
        // Check that the elapsed time is at least the sleep duration.
        // We add a small tolerance (e.g., 95%) to account for minor scheduling variations.
        assert!(elapsed.as_millis() >= (sleep_duration_ms as u128 * 95 / 100));
    }
}
