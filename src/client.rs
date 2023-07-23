use std::{
    io,
    net::{AddrParseError, IpAddr, SocketAddr, UdpSocket},
};

use crate::{
    crypt::{Crypt, Cryptable},
    packet::PACKET_SIZE,
};

const RECEIVE_PORT: u16 = 33740;
const SEND_PORT: u16 = 33739;

pub struct Client {
    socket: UdpSocket,
    destination: SocketAddr,
}

impl Client {
    pub fn new<T: AsRef<str>>(ip_address: T) -> io::Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", RECEIVE_PORT))?;
        let destination: SocketAddr = format!("{}:{}", ip_address.as_ref(), SEND_PORT)
            .parse()
            .expect("Invalid IP address supplied");
        Ok(Self {
            socket,
            destination,
        })
    }

    pub fn start(&self) -> Result<(), io::Error> {
        self.send_heartbeat_packet()?;
        let mut buf = [0u8; PACKET_SIZE];
        loop {
            let bytes_received = &self.socket.recv(&mut buf)?;
            let decrypted = Crypt::decrypt(&mut buf);
            // TODO: do more
            println!("{:#?}", decrypted)
        }
    }

    fn send_heartbeat_packet(&self) -> io::Result<usize> {
        self.socket.send_to(b"A", &self.destination)
    }
}
