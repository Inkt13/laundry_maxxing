pub mod laundry;

use crate::laundry::LaundryMachine;
use strum::IntoEnumIterator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for machine in LaundryMachine::iter() {
        match machine.is_free().await {
            Ok(free) => println!("{machine:?} is Free? {free}"),
            Err(error) => println!("{error:?}")
        }
    }
    Ok(())
}