use std::fmt::Display;

pub struct Icmp {
    r#type: u8,
    code: u8,
    checksum: u16,
    identifier: u16,
    seq_num: u16,

    payload: Vec<u8>,
}

impl Display for Icmp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type: {:#02x}\nCode: {:#02x}\nChecksum: {:#02x}\nIdentifier: {:#02x}\nSequence Number: {:#02x}\nPayload: {:02x?}\n",
            self.r#type, self.code, self.checksum, self.identifier, self.seq_num, self.payload
        )
    }
}

impl Icmp {
    fn increment_seq_num(&mut self) {
        self.seq_num += 1;
    }

    fn ones_complement_sum(x: u16, y: u16) -> u16 {
        let sum = x as u32 + y as u32;
        let carry = (sum >> 16) as u16;

        (sum as u16).wrapping_add(carry)
    }

    fn calculate_checksum(&mut self) {
        let mut checksum: u16 = ((self.r#type as u16) << 8) + (self.code as u16);

        // Checksum field is zero during calculation
        checksum = Self::ones_complement_sum(checksum, self.identifier);
        checksum = Self::ones_complement_sum(checksum, self.seq_num);

        let mut i = 0;
        while i + 1 < self.payload.len() {
            let double = ((self.payload[i] as u16) << 8) + (self.payload[i + 1] as u16);
            checksum = Self::ones_complement_sum(checksum, double);
            i += 2;
        }

        if self.payload.len() & 1 != 0 {
            let double = (self.payload[self.payload.len() - 1] as u16) << 8;
            checksum = Self::ones_complement_sum(checksum, double);
        }

        self.checksum = !checksum;
    }

    pub fn new() -> Self {
        // Hardcoded payload for now (stolen from unix ping payload)
        let payload = vec![
            0xc, 0x51, 0x41, 0x6a, 0x0, 0x0, 0x0, 0x0, 0x16, 0x81, 0x6, 0x0, 0x0, 0x0, 0x0, 0x0,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
            0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b,
            0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
        ];

        Icmp {
            r#type: 0x08,
            code: 0x00,
            checksum: 0x00,
            identifier: std::process::id() as u16,
            seq_num: 0x00,
            payload,
        }
    }

    pub fn serialize(&mut self) -> Vec<u8> {
        self.increment_seq_num();
        self.calculate_checksum();

        let checksum = self.checksum.to_be_bytes();
        let identifier = self.identifier.to_be_bytes();
        let seq_num = self.seq_num.to_be_bytes();

        let mut icmp_packet = vec![
            self.r#type,
            self.code,
            checksum[0],
            checksum[1],
            identifier[0],
            identifier[1],
            seq_num[0],
            seq_num[1],
        ];
        icmp_packet.extend_from_slice(&self.payload);

        icmp_packet
    }
}
