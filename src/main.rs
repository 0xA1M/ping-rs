use std::{
    env,
    mem::MaybeUninit,
    net::{IpAddr, SocketAddr},
    time::Duration,
};

mod icmp;
use icmp::IcmpHeader;
use socket2::{Domain, Protocol, Socket, Type};

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

    let ip: IpAddr = args.iter().nth(1).unwrap().parse().unwrap();
    let address = SocketAddr::new(ip, 0);

    socket.connect(&address.into()).unwrap();

    let mut buffer = [MaybeUninit::<u8>::uninit(); 1024];
    for i in 1..5 {
        let mut icmp_echo_req = IcmpHeader {
            r#type: 0x08,
            code: 0x00,
            checksum: 0x00,
            identifier: std::process::id() as u16,
            seq_num: i,
        };

        let payload = vec![
            0xc, 0x51, 0x41, 0x6a, 0x0, 0x0, 0x0, 0x0, 0x16, 0x81, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
            0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
            0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        ];

        icmp_echo_req.calculate_checksum(&payload);
        println!("{:#x?}", icmp_echo_req);

        let checksum = icmp_echo_req.checksum.to_be_bytes();
        let identifier = icmp_echo_req.identifier.to_be_bytes();
        let seq_num = icmp_echo_req.seq_num.to_be_bytes();

        let mut icmp_packet = vec![
            icmp_echo_req.r#type,
            icmp_echo_req.code,
            checksum[0],
            checksum[1],
            identifier[0],
            identifier[1],
            seq_num[0],
            seq_num[1],
        ];
        icmp_packet.extend_from_slice(&payload);

        socket.send(&icmp_packet).expect("Failed to send message");
        socket.recv(&mut buffer).unwrap();
    }
}
