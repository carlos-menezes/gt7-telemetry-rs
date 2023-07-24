use std::{
    error::Error,
    io,
    net::{AddrParseError, IpAddr, SocketAddr, UdpSocket},
};

use crate::{
    crypt::{Crypt, Cryptable},
    packet::{Packet, HEARTBEAT_PACKET_DATA, PACKET_SIZE},
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

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_heartbeat_packet()?;
        let mut buf = [0u8; PACKET_SIZE];
        loop {
            self.socket.recv(&mut buf)?;
            let decrypted_packet = Crypt::decrypt(&mut buf)?;
            let packet = Packet::parse(decrypted_packet)?;
            println!("{} :: {}", packet.packet_id, packet.suggested_gear)
        }
    }

    fn send_heartbeat_packet(&self) -> io::Result<usize> {
        self.socket
            .send_to(HEARTBEAT_PACKET_DATA, &self.destination)
    }
}
