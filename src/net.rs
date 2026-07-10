use std::{
    io::{self, Read},
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use crate::error::{PingError, Result};

use socket2::{Domain, Protocol, Socket, Type};

pub struct IcmpSocket {
    socket: Socket,
}

impl IcmpSocket {
    pub fn new(timeout: Duration) -> Result<IcmpSocket> {
        let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).map_err(|e| {
            match e.kind() {
                io::ErrorKind::PermissionDenied => PingError::PermissionDenied(e),
                _ => PingError::SocketFailed(e),
            }
        })?;

        socket
            .set_read_timeout(Some(timeout))
            .map_err(PingError::SocketFailed)?;

        Ok(IcmpSocket { socket })
    }

    pub fn connect(&self, ip: IpAddr, port: u16) -> Result<()> {
        let address = SocketAddr::new(ip, port);

        self.socket
            .connect(&address.into())
            .map_err(PingError::SocketFailed)?;

        Ok(())
    }

    pub fn send(&self, icmp_req: &[u8]) -> Result<usize> {
        self.socket.send(icmp_req).map_err(PingError::SocketFailed)
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.socket.read(buf).map_err(PingError::SocketFailed)
    }
}
