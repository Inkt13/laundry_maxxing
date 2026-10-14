pub mod laundry;

use std::time::Duration;

use crate::laundry::LaundryMachine;
use chrono::Local;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcone to laundry maxxer");
    println!("O: machine is free to use");
    println!("X: machine is in use (not available)");
    println!("?: an error happened (check internet connection/turn off VPN)");
    let client = reqwest::Client::builder().no_proxy().build()?;
    loop {
        let local_time = Local::now().format("%T");
        let mut result_text = format!("[{local_time}] (Washers) ").to_string();
        for machine in LaundryMachine::iter() {
            // Add space before dryer
            if machine == LaundryMachine::Dryer {
                result_text += "  (Dryer) "
            }

            match machine.is_free(&client).await {
                Ok(is_free) => result_text += if is_free { "O" } else { "X" },
                Err(_) => result_text += "?",
            }
        }
        println!("{result_text}");
        std::thread::sleep(Duration::from_secs(10));
    }
}
