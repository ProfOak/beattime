use chrono::offset::{FixedOffset, Utc};
use chrono::Timelike;
use clap::Parser;
use std::io::Write;
use std::{thread, time};

static BEAT_DIVISOR: f64 = 86.4;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    continuous: bool,
}

fn main() {
    let args = Args::parse();
    if !args.continuous {
        print!("@{:04}", get_beats());
        return;
    }

    loop {
        let beats = get_beats();
        if its_time(beats) {
            print!("\r@{:04} (it's time)", beats);
        } else {
            print!("\r@{:04}            ", beats);
        }

        std::io::stdout().flush().unwrap();
        thread::sleep(time::Duration::new(1, 0));
    }
}

/// The formula for calculating beat time is as follows:
/// `((UTC+1 time in seconds) / 86.4`
fn get_beats() -> u64 {
    let time = Utc::now().with_timezone(&FixedOffset::east_opt(3600).unwrap());
    let seconds = time.hour() * 3600 + time.minute() * 60 + time.second();
    (seconds as f64 / BEAT_DIVISOR) as u64
}

/// its_time indicates whether or not the hundredths place is an even number or not. This is
/// used by some PSO mechanics.
fn its_time(beats: u64) -> bool {
    (beats / 100) % 2 == 0
}
