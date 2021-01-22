use chrono::Utc;
use signal_hook::{iterator::Signals, consts::TERM_SIGNALS};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Started forever loop: [{}]", Utc::now());
    let mut signals = Signals::new(TERM_SIGNALS)?;

    for sig in signals.forever() {
        println!("Received signal {:?} at [{}]", sig, Utc::now());
        std::process::exit(0);
    }

    Ok(())
}
