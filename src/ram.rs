use log::{debug, error, info, trace};
use rand::{thread_rng, Rng};
use std::mem::offset_of;
use std::thread;
use std::time::{Duration, Instant};

pub fn ram_test(duration: u64, size_mb: usize) {
    if duration == 0 {
        return;
    }
    info!("Running RAM test for {} seconds", duration);
    let buffer_size = size_mb * 1024 * 1024;
    let start = Instant::now();

    let source_data = generate_random_data(buffer_size);

    while start.elapsed().as_secs() < duration {
        let iteration_start = Instant::now();
        for offset in 0..10 {
            match test_memory_with_offset(&source_data, buffer_size, offset) {
                Ok(_) => trace!("Memory test passed starting at offset {}.", offset),
                Err(e) => panic!("Memory test failed at offset {}: {}", offset, e),
            }
        }
        debug!("Iterations done, {:?}", iteration_start.elapsed());

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            error!("RAM Test too long, does not fit in one second");
        }
    }
}

/// Generate a single block of random data for memory testing
fn generate_random_data(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

/// Copy data from a single source buffer to a simulated RAM buffer starting at a given offset
fn test_memory_with_offset(
    source: &[u8],
    test_size: usize,
    start_offset: usize,
) -> Result<(), String> {
    let source_size = source.len();

    // Ensure the test size does not exceed the source size
    if test_size > source_size {
        return Err("Test size cannot exceed source buffer size.".to_string());
    }

    // Allocate the test buffer (simulated RAM)
    let mut test_buffer = vec![0u8; test_size];

    // Copy data from the source buffer, wrapping around if necessary
    for i in 0..test_size {
        test_buffer[i] = source[(start_offset + i) % source_size];
    }

    // Verify that the copied data matches the expected source data
    for i in 0..test_size {
        if test_buffer[i] != source[(start_offset + i) % source_size] {
            return Err(format!(
                "Memory verification failed at index {}: expected {}, got {}",
                i,
                source[(start_offset + i) % source_size],
                test_buffer[i]
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_with_random_data() {
        let source_size = 1024; // 1KB of random data
        let test_size = 256; // Test with 256 bytes each time
        let source_data = generate_random_data(source_size);

        // Perform multiple tests starting from different offsets
        for offset in [0, 128, 512, 800].iter() {
            match test_memory_with_offset(&source_data, test_size, *offset) {
                Ok(_) => info!("Memory test passed starting at offset {}.", offset),
                Err(e) => panic!("Memory test failed at offset {}: {}", offset, e),
            }
        }
    }
}
