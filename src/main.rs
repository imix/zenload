use clap::Parser;
use dotenv::dotenv;
use log::{error, info};
use rand::Rng;
use std::thread;
use std::time::{Duration, Instant};

mod config;
mod cpu;
mod disk;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long, default_value = "5")]
    cpu_secs: u64,
    #[clap(short, long)]
    file: String,
}

#[derive(Debug)]
struct ConfigArgs {
    verbose: bool,
    cpu_secs: u64,
    file: String,
}

impl From<Args> for ConfigArgs {
    fn from(args: Args) -> Self {
        ConfigArgs {
            verbose: args.verbose,
            cpu_secs: args.cpu_secs,
            file: args.file,
        }
    }
}

fn ram_test(duration: u64, size_mb: usize) {
    if duration == 0 {
        return;
    }
    info!("Running RAM test for {} seconds", duration);
    let target_allocations_per_sec = 10; // Fixed workload
    let buffer_size = size_mb * 1024 * 1024;
    let start = Instant::now();

    while start.elapsed().as_secs() < duration {
        let iteration_start = Instant::now();
        for _ in 0..target_allocations_per_sec {
            let mut vec = vec![0u8; buffer_size];
            for i in 0..vec.len() {
                vec[i] = rand::thread_rng().gen();
            }
        }

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            error!("RAM Test too long, does not fit in one second");
        }
    }
}

fn gpu_test(duration: u64) {
    // Placeholder for a GPU test using wgpu or other libraries
    info!("Running GPU test...");
    info!(
        "GPU test started for {} seconds. (Implement as needed)",
        duration
    );
    thread::sleep(Duration::from_secs(duration));
}

fn main() {
    let args = Args::parse();
    let config_args: ConfigArgs = args.into();

    dotenv().ok();
    if config_args.verbose {
        env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .init();
    } else {
        env_logger::init();
    }

    // load zenario from yml file
    let zenario = config::load_zenario(&config_args.file).expect("Failed to load zenario");

    info!("Starting load scenario...");
    for test in zenario.load_zenario {
        match test {
            config::LoadZenario::Test(t) => {
                println!("Running test: {}", t.name);
                match t.test_type.as_str() {
                    "cpu" => cpu::cpu_test(
                        t.duration.expect("duration is mandatory"),
                        t.operation.expect("operation is mandatory"),
                    ),
                    "disk" => disk::disk_io_test(
                        &t.file_path.unwrap(),
                        t.duration.expect("msg is mandatory"),
                    ),
                    // "ram" => ram_test(ram_duration, t.data_size.unwrap_or(0) as usize),
                    // "gpu" => gpu_test(gpu_duration),
                    _ => error!("Unknown test type: {}", t.test_type),
                }
            }
            config::LoadZenario::ParallelGroup { parallel_group } => {
                println!("Running parallel tests:");
                for t in parallel_group {
                    println!(" - {}", t.name);
                    // Handle parallel test
                }
            }
        }
    }

    info!("Load scenario complete.");
}
