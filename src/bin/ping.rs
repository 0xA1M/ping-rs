use std::{env, time::Duration};

use ping_rs::{config::PingConfig, ping};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ping-rs <IP>");
        return;
    }

    let config = PingConfig::new(
        args.get(1).unwrap(),
        4,
        Duration::from_secs(1),
        Duration::from_secs(1),
    )
    .unwrap();

    ping(&config).unwrap()
}
