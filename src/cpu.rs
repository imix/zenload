use log::{debug, error, info, trace};
use std::thread;
use std::time::{Duration, Instant};

pub fn cpu_test(duration: u64, test_type: String) {
    if duration == 0 {
        return;
    }
    info!("Running CPU test for {} seconds", duration);
    let start = Instant::now();

    while start.elapsed().as_secs() < duration {
        info!("CPU Test loop");
        let iteration_start = Instant::now();

        let res = match test_type.as_str() {
            "int24" => generate_integer_load_2024(),
            _ => error!("Unknown CPU test type: {}", test_type),
        };

        debug!("Iterations done, {:?}", iteration_start.elapsed());
        trace!("Got, {:?}", res); // make sure the result is not optimized away

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            error!("CPU Test too long, does not fit in one second");
        }
    }
}

fn generate_integer_load_2024() {
    let mut iterations = 0;
    let mut a: u64 = 0;
    let target_iterations_per_sec = 1_000_000; // Fixed workload
    while iterations < target_iterations_per_sec {
        let s: u64 = (1..100).map(|x| iterations / x).sum();
        a += s;
        iterations += 1;
    }
}
