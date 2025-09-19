use std::{
    env, io,
    mem::MaybeUninit,
    net::{SocketAddr, ToSocketAddrs},
    slice,
    time::Duration,
};

use socket2::{Domain, MaybeUninitSlice, Protocol, Socket, Type};

mod icmp;
use icmp::ICMPPacket;

fn make_icmp_req(addr: SocketAddr, count: Option<usize>, timeout: Option<u64>) -> io::Result<()> {
    let socket = if addr.is_ipv4() {
        Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?
    } else {
        Socket::new(Domain::IPV6, Type::RAW, Some(Protocol::ICMPV6))?
    };

    let timeout = timeout.unwrap_or(1);
    let count = count.unwrap_or(4);

    socket.set_read_timeout(Some(Duration::from_secs(timeout)))?;

    let identifier = std::process::id() as u16;
    let payload_size = 32usize;

    println!("PING {}: {} data bytes", addr.ip(), payload_size);

    for i in 0..count {
        let sequence_number = i as u16;
        let mut icmp_packet = ICMPPacket::new(addr, identifier, sequence_number, payload_size);

        icmp_packet.calculate_checksum();
        let packet_bytes = icmp_packet.serialize();

        let bytes_sent = socket.send_to(&packet_bytes, &addr.into())?;
        println!(
            "Sent {} bytes (ID: {}, Seq: {}) to {}",
            bytes_sent,
            icmp_packet.identifier,
            icmp_packet.sequence_number,
            addr.ip()
        );

        let mut buf = [MaybeUninit::<u8>::zeroed(); 1024];
        let (bytes_recv, t_addr) = socket.recv_from(&mut MaybeUninitSlice::new(&mut buf))?;

        let buf = unsafe { slice::from_raw_parts(buf.as_ptr() as *const u8, bytes_recv) };
        let icmp_packet = ICMPPacket::deserialize(buf, bytes_recv);

        println!(
            "Received {} bytes (Type: {}, Code: {}, ID: {}, Seq: {}) from {}",
            bytes_recv,
            icmp_packet.icmp_type,
            icmp_packet.code,
            icmp_packet.identifier,
            icmp_packet.sequence_number,
            t_addr.as_socket().unwrap().ip(),
        );
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let prog_name = args.get(0).unwrap();
        eprintln!("Usage: {} <ip/hostname>", prog_name);
        return;
    }

    let hostname = args.get(1).unwrap();
    let addr_str = format!("{}:0", hostname);

    match addr_str.to_socket_addrs() {
        Ok(addrs) => {
            let mut target_addr: Option<SocketAddr> = None;
            for addr in addrs {
                if addr.is_ipv4() {
                    target_addr = Some(addr);
                    break;
                }
            }

            if let Some(addr) = target_addr {
                println!("Attempting to ping address: {}", addr.ip());
                if let Err(e) = make_icmp_req(addr, None, None) {
                    eprintln!(
                        "An error occurred during ICMP request to {}: {}",
                        addr.ip(),
                        e
                    );
                }
            } else {
                eprintln!("No IPv4 address found for hostname: {}", hostname);
            }
        }
        Err(e) => {
            eprintln!("Error resolving address: '{}': {}", hostname, e)
        }
    }
}
