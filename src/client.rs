use std::{
    error::Error,
    io::{self, ErrorKind},
    net::{AddrParseError, IpAddr, SocketAddr, UdpSocket},
    sync::{mpsc::Sender, Arc, Mutex},
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
    channel_tx: Arc<Mutex<Sender<Packet>>>,
}

impl Client {
    pub fn new<T: AsRef<str>>(
        ip_address: T,
        channel_tx: Arc<Mutex<Sender<Packet>>>,
    ) -> io::Result<Self> {
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", RECEIVE_PORT))?;
        socket
            .set_nonblocking(true)
            .expect("could not set socket to nonblocking");
        let destination: SocketAddr = format!("{}:{}", ip_address.as_ref(), SEND_PORT)
            .parse()
            .expect("Invalid IP address supplied");
        Ok(Self {
            socket,
            destination,
            channel_tx,
        })
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.send_heartbeat_packet()?;
        let mut buf = [0u8; PACKET_SIZE];
        loop {
            let data = self.socket.recv(&mut buf);
            match data {
                Ok(_) => {
                    let decrypted_packet = Crypt::decrypt(&mut buf)?;
                    let packet = Packet::parse(decrypted_packet)?;
                    println!("-> Packet #{} received", packet.packet_id);
                    if &packet.packet_id % 100 == 0 {
                        self.send_heartbeat_packet()?;
                    }

                    self.channel_tx
                        .lock()
                        .expect("channel_tx muted poisoned")
                        .send(packet)
                        .expect("failed to send packet");
                }
                Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                    println!("Something went wrong: {}", err)
                }
                // Do nothing otherwise.
                _ => {}
            }
        }
    }

    fn send_heartbeat_packet(&self) -> io::Result<usize> {
        self.socket
            .send_to(HEARTBEAT_PACKET_DATA, &self.destination)
    }
}
