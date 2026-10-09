pub mod laundry;

use crate::laundry::LaundryMachine;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    loop {
        let mut result_text= String::new();
        for machine in LaundryMachine::iter() {
            // Add space before dryer
            if machine == LaundryMachine::Dryer {
                result_text += " "
            }

            match machine.is_free(&client).await {
                Ok(is_free) => result_text += if is_free {"O"} else {"X"},
                Err(error) => {
                    println!("Fetching {machine:?} status error: {error:?}");
                    result_text += "?"
                }
            }
        }
        println!("{result_text}")
    }
}