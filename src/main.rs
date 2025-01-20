use clap::Parser;
use dotenv::dotenv;
use log::{error, info};
use std::thread;
use std::time::Duration;

mod config;
mod cpu;
mod disk;
mod ram;

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
                        t.duration.expect("duration is mandatory"),
                    ),
                    "ram" => ram::ram_test(
                        t.duration.expect("duration is mandatory"),
                        // read t.data_size String as usize
                        t.data_size
                            .expect("size is mandatory")
                            .trim()
                            .parse::<usize>()
                            .expect("Invalid size"),
                    ),
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
