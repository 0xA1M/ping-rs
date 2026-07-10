#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum IcmpType {
    EchoRequest = 0x8,
    EchoReply = 0x0,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub struct IcmpHeader {
    pub icmp_type: IcmpType,
    pub code: u8,
    pub identifier: u16,
    pub seq_num: u16,
}

impl From<u8> for IcmpType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => Self::EchoReply,
            0x8 => Self::EchoRequest,
            _ => Self::Unknown,
        }
    }
}
