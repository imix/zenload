use log::{debug, error, info, trace};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

pub fn disk_io_test(file_path: &str, duration: u64) {
    if duration == 0 {
        return;
    }
    info!("Running Disk I/O test for {} seconds", duration);
    let start = Instant::now();
    let buffer_size = 1024 * 1024; // 1 MB buffer
    let target_writes_per_sec = 10; // Fixed workload

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

    while start.elapsed().as_secs() < duration {
        let iteration_start = Instant::now();
        for _ in 0..target_writes_per_sec {
            let data = vec![0u8; buffer_size];
            file.write_all(&data).unwrap();
            file.sync_all().unwrap();
        }
        debug!("Iterations done, {:?}", iteration_start.elapsed());

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            error!("Disk Test too long, does not fit in one second");
        }
    }

    // Cleanup
    std::fs::remove_file(file_path).unwrap();
}
