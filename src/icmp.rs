#[derive(Debug)]
pub struct IcmpHeader {
    pub r#type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub seq_num: u16,
}

impl IcmpHeader {
    fn ones_complement_sum(x: u16, y: u16) -> u16 {
        let sum = x as u32 + y as u32;
        let carry = (sum >> 16) as u16;

        (sum as u16).wrapping_add(carry)
    }

    pub fn calculate_checksum(&mut self, payload: &[u8]) {
        let payload_len = payload.len();
        let mut checksum: u16 = ((self.r#type as u16) << 8) + (self.code as u16);

        // Checksum field is zero during calculation
        checksum = Self::ones_complement_sum(checksum, self.identifier);
        checksum = Self::ones_complement_sum(checksum, self.seq_num);

        let mut i = 0;
        while i + 1 < payload_len {
            let double = ((payload[i] as u16) << 8) + (payload[i + 1] as u16);
            checksum = Self::ones_complement_sum(checksum, double);
            i += 2;
        }

        if payload_len & 1 != 0 {
            let double = (payload[payload_len - 1] as u16) << 8;
            checksum = Self::ones_complement_sum(checksum, double);
        }

        self.checksum = !checksum;
    }
}
