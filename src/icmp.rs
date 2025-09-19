use std::net::SocketAddr;

pub struct ICMPPacket {
    pub icmp_type: u8,
    pub code: u8,
    pub checksum: u16,
    pub identifier: u16,
    pub sequence_number: u16,
    pub payload: Vec<u8>,
}

impl ICMPPacket {
    pub fn new(
        addr: SocketAddr,
        identifier: u16,
        sequence_number: u16,
        payload_size: usize,
    ) -> Self {
        let (icmp_type, code) = Self::get_packet_type(addr.is_ipv4());

        // Ensure a minimum payload size to accommodate standard ping behavior.
        let payload_size = payload_size.max(16);
        let payload = vec![0; payload_size];

        ICMPPacket {
            icmp_type,
            code,
            checksum: 0, // Checksum is calculated later
            identifier,
            sequence_number,
            payload,
        }
    }

    pub fn calculate_checksum(&mut self) {
        let mut sum = 0u32;

        // ICMP header fields
        // Type and Code are combined into a 16-bit word
        sum += ((self.icmp_type as u32) << 8) | (self.code as u32);
        // Checksum field is 0 during calculation
        sum += self.identifier as u32;
        sum += self.sequence_number as u32;

        // Payload
        let mut i = 0;
        while i < self.payload.len() {
            let high = self.payload[i] as u32;
            let low = if i + 1 < self.payload.len() {
                self.payload[i + 1] as u32
            } else {
                0
            };

            sum += (high << 8) | low;
            i += 2;
        }

        // Fold 32-bit sum to 16 bits
        while (sum >> 16) != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        self.checksum = !(sum as u16);
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut packet_bytes = Vec::new();

        packet_bytes.push(self.icmp_type);
        packet_bytes.push(self.code);
        packet_bytes.extend_from_slice(&self.checksum.to_be_bytes());
        packet_bytes.extend_from_slice(&self.identifier.to_be_bytes());
        packet_bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        packet_bytes.extend_from_slice(&self.payload);

        packet_bytes
    }

    pub fn deserialize(bytes: &[u8], len: usize) -> Self {
        let mut icmp_packet = ICMPPacket {
            icmp_type: 0,
            code: 0,
            checksum: 0,
            identifier: 0,
            sequence_number: 0,
            payload: Vec::new(),
        };

        // Check if the buffer is too short to even contain an IP header
        if len < 20 {
            eprintln!("Received packet too short to be valid (len: {})", len);
            return icmp_packet;
        }

        // Extract IP header length (IHL) from the first byte
        // The first byte contains Version (bits 0-3) and IHL (bits 4-7)
        let ip_header_len = (bytes[0] & 0x0F) as usize * 4; // IHL is in 32-bit words, so multiply by 4 for bytes

        if ip_header_len < 20 || ip_header_len > len {
            eprintln!(
                "Invalid IP header length or packet too short after IP header (IP header len: {}, total len: {})",
                ip_header_len, len
            );
            return icmp_packet;
        }

        // The ICMP packet starts *after* the IP header
        let icmp_start_index = ip_header_len;

        // An ICMP header is 8 bytes.
        if len - icmp_start_index < 8 {
            eprintln!("Received packet too short for ICMP header after IP header");
            return icmp_packet;
        }

        // ICMP Headers
        icmp_packet.icmp_type = bytes[icmp_start_index];
        icmp_packet.code = bytes[icmp_start_index + 1];
        icmp_packet.checksum =
            u16::from_be_bytes([bytes[icmp_start_index + 2], bytes[icmp_start_index + 3]]);
        icmp_packet.identifier =
            u16::from_be_bytes([bytes[icmp_start_index + 4], bytes[icmp_start_index + 5]]);
        icmp_packet.sequence_number =
            u16::from_be_bytes([bytes[icmp_start_index + 6], bytes[icmp_start_index + 7]]);

        // Payload
        let payload_start_index = icmp_start_index + 8;
        if payload_start_index < len {
            icmp_packet.payload = Vec::from(&bytes[payload_start_index..len]);
        } else {
            icmp_packet.payload = Vec::new();
        }

        icmp_packet
    }

    fn get_packet_type(is_ipv4: bool) -> (u8, u8) {
        if is_ipv4 { (8, 0) } else { (128, 0) }
    }
}
