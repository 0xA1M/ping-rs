use std::net::{IpAddr, Ipv4Addr};

use crate::error::{PingError, Result};
use crate::utils::calculate_checksum;

use super::types::{IcmpHeader, IcmpType};

pub const ICMP_ECHO_CODE: u8 = 0x0;
pub const DEFAULT_PAYLOAD: [u8; 56] = [
    0xc, 0x51, 0x41, 0x6a, 0x0, 0x0, 0x0, 0x0, 0x16, 0x81, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0, 0x10,
    0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30,
    0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
];

#[derive(Debug)]
pub struct IcmpRequest {
    pub header: IcmpHeader,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct IcmpReply {
    pub source: IpAddr,
    pub ttl: u8,
    pub header: IcmpHeader,
    pub payload: Vec<u8>,
}

impl IcmpRequest {
    pub fn new(
        icmp_type: IcmpType,
        code: u8,
        identifier: u16,
        seq_num: u16,
        payload: Vec<u8>,
    ) -> Self {
        IcmpRequest {
            header: IcmpHeader {
                icmp_type,
                code,
                identifier,
                seq_num,
            },
            payload,
        }
    }

    pub fn set_seq_num(&mut self, seq_num: u16) {
        self.header.seq_num = seq_num;
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut icmp_packet = Vec::with_capacity(8 + self.payload.len());

        icmp_packet.push(self.header.icmp_type as u8);
        icmp_packet.push(self.header.code);
        icmp_packet.extend_from_slice(&0u16.to_be_bytes());
        icmp_packet.extend_from_slice(&self.header.identifier.to_be_bytes());
        icmp_packet.extend_from_slice(&self.header.seq_num.to_be_bytes());
        icmp_packet.extend_from_slice(&self.payload);

        let checksum = calculate_checksum(&icmp_packet);
        icmp_packet[2..4].copy_from_slice(&checksum.to_be_bytes());

        icmp_packet
    }

    pub fn deserialize(&self, data: &[u8]) -> Result<IcmpReply> {
        if data.len() < 28 {
            return Err(PingError::IcmpProtocol("truncated".to_string()));
        }

        let ttl = data[8];
        let source_raw = u32::from_be_bytes([data[12], data[13], data[14], data[15]]);
        let source = IpAddr::V4(Ipv4Addr::from_bits(source_raw));

        let header = IcmpHeader {
            icmp_type: data[20].into(),
            code: data[21],
            identifier: u16::from_be_bytes([data[24], data[25]]),
            seq_num: u16::from_be_bytes([data[26], data[27]]),
        };
        let payload = data[28..].to_vec();

        let checksum = u16::from_be_bytes([data[22], data[23]]);
        let calculated_checksum = calculate_checksum(&data[20..]);

        if checksum != calculated_checksum {
            return Err(PingError::IcmpProtocol("invalid checksum".to_string()));
        }

        if self.header.identifier != header.identifier {
            return Err(PingError::IcmpProtocol("invalid identifier".to_string()));
        }

        Ok(IcmpReply {
            source,
            ttl,
            header,
            payload,
        })
    }
}
