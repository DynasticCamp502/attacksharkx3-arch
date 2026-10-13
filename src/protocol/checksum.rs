//! 16-bit big-endian payload checksum calculation and verification.


/// Compute the 16-bit big-endian sum of payload bytes from index 3 up to `check_offset`.
pub fn compute_checksum(buf: &[u8], check_offset: usize) -> u16 {
    let mut sum: u32 = 0;
    for &b in &buf[3..check_offset] {
        sum += b as u32;
    }
    (sum & 0xFFFF) as u16
}

/// Apply checksum to a mutable buffer at `check_offset` (2 bytes, big-endian).
pub fn apply_checksum(buf: &mut [u8], check_offset: usize) {
    let check = compute_checksum(buf, check_offset);
    buf[check_offset] = ((check >> 8) & 0xFF) as u8;
    buf[check_offset + 1] = (check & 0xFF) as u8;
}

/// Verify checksum of a buffer at `check_offset`.
pub fn verify_checksum(buf: &[u8], check_offset: usize) -> bool {
    if buf.len() < check_offset + 2 {
        return false;
    }
    let expected = compute_checksum(buf, check_offset);
    let actual = ((buf[check_offset] as u16) << 8) | (buf[check_offset + 1] as u16);
    expected == actual
}

/// Compute 16-bit big-endian checksum for a 128-byte macro block (bytes 0..126).
pub fn compute_macro_checksum(block: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for &b in &block[0..126] {
        sum += b as u32;
    }
    (sum & 0xFFFF) as u16
}

/// Apply macro checksum to bytes 126..128 of a 128-byte block.
pub fn apply_macro_checksum(block: &mut [u8]) {
    let check = compute_macro_checksum(block);
    block[126] = ((check >> 8) & 0xFF) as u8;
    block[127] = (check & 0xFF) as u8;
}

/// Verify macro checksum.
pub fn verify_macro_checksum(block: &[u8]) -> bool {
    if block.len() < 128 {
        return false;
    }
    let expected = compute_macro_checksum(block);
    let actual = ((block[126] as u16) << 8) | (block[127] as u16);
    expected == actual
}
