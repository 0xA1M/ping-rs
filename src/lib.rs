pub mod config;
pub mod error;
pub mod icmp;
pub mod net;
pub mod stats;
pub mod utils;

use std::time::Instant;

use config::PingConfig;
use icmp::IcmpRequest;
use net::IcmpSocket;

use crate::{
    error::Result,
    icmp::IcmpReply,
    stats::{PingReply, PingStats},
};

pub fn ping(config: &PingConfig) -> Result<()> {
    let mut icmp_socket = IcmpSocket::new(config.timeout)?;
    icmp_socket.connect(config.target, 0)?;

    println!(
        "PING {} ({}) {}({}) bytes of data.",
        config.target,
        config.target,
        icmp::DEFAULT_PAYLOAD.len(),
        icmp::DEFAULT_PAYLOAD.len() + 28, // IP + ICMP header size
    );

    let mut icmp = IcmpRequest::new(
        icmp::IcmpType::EchoRequest,
        icmp::ICMP_ECHO_CODE,
        config.identifier,
        0,
        icmp::DEFAULT_PAYLOAD.into(),
    );
    let mut buffer = [0u8; 1024];

    let mut stats = PingStats::new(config.target, None);
    for i in 1..=config.count {
        let next = Instant::now() + config.interval;
        icmp.set_seq_num(i as u16);

        let start_time = Instant::now();
        icmp_socket.send(&icmp.serialize())?;
        stats.record_send();

        let bytes_read = icmp_socket.recv(&mut buffer)?;
        let end_time = Instant::now();

        let reply: IcmpReply = icmp.deserialize(&buffer[..bytes_read])?;

        let reply = PingReply {
            source: config.target,
            seq: i as u16,
            ttl: reply.ttl,
            bytes: bytes_read - 20,
            rtt: end_time - start_time,
        };

        reply.print();

        stats.record_reply(reply);

        let remaining = next.saturating_duration_since(Instant::now());
        std::thread::sleep(remaining);
    }

    println!("\n{}", stats);

    Ok(())
}
