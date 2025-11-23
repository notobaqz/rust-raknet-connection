use std::{env, net::UdpSocket, time::{Duration, Instant}};
mod proto {
    pub mod connection {
        pub mod unconnected_ping;
        pub mod handshake;
    }
}

use crate::proto::connection::unconnected_ping::{build_unconnected_ping, parse_unconnected_pong};
use crate::proto::connection::handshake::{
    openconn1, parse_openconn1,
    openconn2, parse_openconn2,
};

fn main() -> std::io::Result<()> {
    let server_addr = env::args().nth(1).expect("usage: cargo run <ip:port>");

    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;

    let start = Instant::now();
    socket.send_to(&build_unconnected_ping(), &server_addr)?;
    let mut buf = [0u8; 1500];
    let (size, src) = socket.recv_from(&mut buf)?;
    println!("received {} bytes from {}", size, src);

    let (server_guid, motd) = parse_unconnected_pong(&buf[..size]).expect("failed to parse unconnected pong");
    println!("server guid: {}", server_guid);
    println!("motd: {}", motd);

    socket.send_to(&openconn1(), &server_addr)?;
    let (size, _) = socket.recv_from(&mut buf)?;
    let (server_guid, cookie, mtu) = parse_openconn1(&buf[..size]).expect("failed to parse open connection reply 1");

    println!("open connection reply 1 received");
    println!("server guid: {}", server_guid);
    println!("cookie: {}", cookie);
    println!("mtu: {}", mtu);

    let client_guid = rand::random::<u64>();
    socket.send_to(&openconn2(cookie, mtu, client_guid), &server_addr)?;
    let (size, _) = socket.recv_from(&mut buf)?;
    let (server_guid, _mtu_reply_2, security) = parse_openconn2(&buf[..size]).expect("failed to parse open connection reply 2");

    println!("open connection reply 2 received");
    println!("server guid: {}", server_guid);
    println!("security enabled: {}", security);
    println!("connection handshake complete in {} seconds.", start.elapsed().as_secs());

    let mut bytes_sent = 0;
    while start.elapsed().as_secs() < 5 {
        let data = vec![0u8; mtu as usize];
        socket.send_to(&data, &server_addr)?;
        bytes_sent += data.len();
    }
    println!("sent {} bytes in {} seconds.", bytes_sent, start.elapsed().as_secs());

    Ok(())
}
