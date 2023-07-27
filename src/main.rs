use std::{
    os::unix::thread,
    sync::{Arc, Mutex},
};

use client::Client;

use packet::Packet;

mod client;
mod crypt;
mod errors;
mod packet;
mod system;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<Packet>();

    let channel_tx = Arc::new(Mutex::new(tx));
    let client = Client::new("192.168.1.154", channel_tx).unwrap();
    let client_thread = std::thread::spawn(move || {
        client.start().unwrap();
    });

    // let channel_rx = Arc::new(Mutex::new(rx));
    let system = system::init();
    system.run(rx);

    client_thread.join().unwrap();
}
