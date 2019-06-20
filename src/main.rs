use chrono::Utc;
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    println!("Started forever loop: [{}]", Utc::now());
    let signals = Signals::new(&[SIGINT, SIGTERM])?;

    for sig in signals.forever() {
        println!("Received signal {:?} at [{}]", sig, Utc::now());
        std::process::exit(0);
    }

    Ok(())
}
