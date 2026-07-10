use std::time::Duration;

pub fn duration_to_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

pub fn ones_complement_sum(x: u16, y: u16) -> u16 {
    let sum = x as u32 + y as u32;
    let carry = (sum >> 16) as u16;

    (sum as u16).wrapping_add(carry)
}

pub(crate) fn calculate_checksum(data: &[u8]) -> u16 {
    let mut checksum: u16 = u16::from_be_bytes([data[0], data[1]]);

    let identifier = u16::from_be_bytes([data[4], data[5]]);
    let seq_num = u16::from_be_bytes([data[6], data[7]]);
    let payload = data[8..].to_vec();

    checksum = ones_complement_sum(checksum, identifier);
    checksum = ones_complement_sum(checksum, seq_num);

    let mut chunks = payload.chunks_exact(2);
    for chunk in &mut chunks {
        let word = u16::from_be_bytes([chunk[0], chunk[1]]);
        checksum = ones_complement_sum(checksum, word);
    }

    if let Some(&byte) = chunks.remainder().first() {
        let word = u16::from_be_bytes([byte, 0]);
        checksum = ones_complement_sum(checksum, word);
    }

    checksum
}
