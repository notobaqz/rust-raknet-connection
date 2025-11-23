use rand::random;

pub const magic: [u8; 16] = [
    0x00, 0xff, 0xff, 0x00,
    0xfe, 0xfe, 0xfe, 0xfe,
    0xfd, 0xfd, 0xfd, 0xfd,
    0x12, 0x34, 0x56, 0x78,
];

pub fn build_unconnected_ping() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(0x01);
    let client_time = rand::random::<u64>();
    buf.extend_from_slice(&client_time.to_be_bytes());
    buf.extend_from_slice(&magic);
    buf
}

pub fn parse_unconnected_pong(data: &[u8]) -> Option<(u64, String)> {
    if data.len() < 35 || data[0] != 0x1c {
        return None;
    }
    let server_guid = u64::from_be_bytes(data[9..17].try_into().ok()?);
    if &data[17..33] != magic {
        return None;
    }
    let str_len = u16::from_be_bytes(data[33..35].try_into().ok()?) as usize;
    if data.len() < 35 + str_len {
        return None;
    }
    let motd = String::from_utf8_lossy(&data[35..35 + str_len]).to_string();
    Some((server_guid, motd))
}
