pub fn build_startup_packet(user: &str, db: &str) -> Vec<u8> {
    let mut packet = vec![];
    // Example: version 3.0 = 196608
    packet.extend(&[0, 0, 0, 0]); // Placeholder for length
    packet.extend(&[0x00, 0x03, 0x00, 0x00]);
    packet.extend(user.as_bytes());
    packet.push(0);
    packet.extend(b"user");
    packet.push(0);
    packet.extend(db.as_bytes());
    packet.push(0);
    packet.push(0);
    let len = packet.len() as u32;
    packet[0..4].copy_from_slice(&len.to_be_bytes());
    packet
}