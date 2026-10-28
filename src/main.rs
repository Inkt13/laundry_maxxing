pub mod laundry;

use std::{
    fs::{self, OpenOptions},
    time::{Duration, Instant},
};

use chrono::Utc;
use colored_text::Colorize;
use laundry::LaundryMachine;
use std::error::Error;
use strum::IntoEnumIterator;
use tokio::time::sleep;

const POLL_INTERVAL: Duration = Duration::from_secs(5 * 60);
const CSV_PATH: &str = "data/laundry.csv";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "Welcome to laundry maxxer".yellow());
    println!("{}: machine is free", "O".green());
    println!("{}: machine is occupied", "X".red());
    println!("{}: status could not be determined", "?".yellow());

    fs::create_dir_all("data")?;

    let csv_exists = fs::metadata(CSV_PATH)
        .map(|metadata| metadata.len() > 0)
        .unwrap_or(false);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(CSV_PATH)?;

    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);

    if !csv_exists {
        writer.write_record([
            "timestamp_utc",
            "machine",
            "status",
            "request_ms",
            "http_status",
            "error",
        ])?;
        writer.flush()?;
    }

    let client = reqwest::Client::builder()
        .no_proxy()
        .timeout(Duration::from_secs(15))
        .build()?;

    loop {
        let cycle_start = Instant::now();
        let timestamp = Utc::now().to_rfc3339();

        println!("\n[{}] Polling machines", timestamp);

        for machine in LaundryMachine::iter() {
            let request_start = Instant::now();
            let result = machine.is_free(&client).await;
            let request_ms = request_start.elapsed().as_millis().to_string();

            let (status, http_status, error) = match result {
                Ok(true) => {
                    println!("{}: {}", machine_name(machine), "O".green());
                    ("free", "200", String::new())
                }
                Ok(false) => {
                    println!("{}: {}", machine_name(machine), "X".red());
                    ("occupied", "200", String::new())
                }
                Err(err) => {
                    println!("{}: {} ({})", machine_name(machine), "?".yellow(), err);

                    ("unknown", "", err.to_string())
                }
            };

            writer.write_record([
                timestamp.as_str(),
                machine_name(machine),
                status,
                request_ms.as_str(),
                http_status,
                error.as_str(),
            ])?;
        }

        // Persist this cycle's observations before sleeping.
        writer.flush()?;

        println!(
            "Cycle completed in {:.2}s. Next poll in 5 minutes.",
            cycle_start.elapsed().as_secs_f64()
        );

        sleep(POLL_INTERVAL).await;
    }
}

fn machine_name(machine: LaundryMachine) -> &'static str {
    match machine {
        LaundryMachine::Washer1 => "Washer1",
        LaundryMachine::Washer2 => "Washer2",
        LaundryMachine::Washer3 => "Washer3",
        LaundryMachine::Washer4 => "Washer4",
        LaundryMachine::Washer5 => "Washer5",
        LaundryMachine::Dryer => "Dryer",
    }
}
