use std::{
    env,
    mem::MaybeUninit,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use socket2::{Domain, Protocol, Socket, Type};

mod icmp;
use icmp::Icmp;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: ping-rs <IP>");
        return;
    }

    let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))
        .expect("Failed to create network socket");

    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("Failed to configure socket");

    let ip: IpAddr = args.get(1).unwrap().parse().unwrap();
    let address = SocketAddr::new(ip, 0);

    socket.connect(&address.into()).unwrap();

    let mut icmp = Icmp::default();
    let mut buffer = [MaybeUninit::<u8>::uninit(); 1024];

    for _ in 1..5 {
        // Increament icmp request packet's sequence number
        icmp.increment_seq_num();

        socket
            .send(&icmp.serialize())
            .expect("Failed to send message");

        socket.recv(&mut buffer).unwrap();
    }
}
