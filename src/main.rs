use std::{env, net::Ipv4Addr};

fn main() {
    if env::args().len() < 2 {
        eprintln!("Exprected IP");
        return;
    }

    let ip = env::args()
        .nth(1)
        .unwrap()
        .parse::<Ipv4Addr>()
        .expect("Not a valid IPv4 address!");

    println!("Parsed ip: {ip}")
}
