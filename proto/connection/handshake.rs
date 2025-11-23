use crate::proto::connection::unconnected_ping::magic;

pub fn openconn1() -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(0x05);
    buf.extend_from_slice(&magic);
    buf.push(0x0b);

    let padding_len = 1464 - 1 - 16 - 1;
    buf.extend(std::iter::repeat(0).take(padding_len));

    buf
}

pub fn parse_openconn1(data: &[u8]) -> Option<(u64, u32, u16)> {
    if data.len() < 27 || data[0] != 0x06 || &data[1..17] != magic {
        return None;
    }

    let guild = u64::from_be_bytes(data[17..25].try_into().ok()?);
    let serversec = data[25] != 0;
    let mut index = 26;
    let cookie = if serversec {
        if data.len() < 30 {
            return None;
        }
        let c = u32::from_be_bytes(data[26..30].try_into().ok()?);
        index = 30;
        c
    } else {
        0
    };
    if data.len() < index + 2 {
        return None;
    }
    let mtu = u16::from_be_bytes(data[index..index + 2].try_into().ok()?);
    Some((guild, cookie, mtu))
}

pub fn openconn2(cookie: u32, mtu: u16, client_guid: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(0x07);
    buf.extend_from_slice(&magic);
    buf.extend_from_slice(&cookie.to_be_bytes());
    buf.push(0); 
    buf.extend_from_slice(&[4, 0, 0, 0, 0, 0, 0]); 
    buf.extend_from_slice(&mtu.to_be_bytes());
    buf.extend_from_slice(&client_guid.to_be_bytes());
    buf
}

pub fn parse_openconn2(data: &[u8]) -> Option<(u64, u16, bool)> {
    if data.len() < 35 || data[0] != 0x08 || &data[1..17] != magic {
        return None;
    }
    let guild = u64::from_be_bytes(data[17..25].try_into().ok()?);

    let mtu = u16::from_be_bytes(data[32..34].try_into().ok()?);
    let security = data[34] != 0;
    Some((guild, mtu, security))
}
