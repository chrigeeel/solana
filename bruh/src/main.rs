use std::net::UdpSocket;
use std::io;


use {
    solana_ledger::shred::{
        max_entries_per_n_shred, max_ticks_per_n_shreds, ProcessShredsStats, ReedSolomonCache,
        Shred, ShredFlags, Shredder, DATA_SHREDS_PER_FEC_BLOCK, LEGACY_SHRED_DATA_CAPACITY,
    },
};

fn main() -> io::Result<()> {
    // Bind the socket to a local address
    let socket = UdpSocket::bind("127.0.0.1:8002")?;
    println!("Listening on {}", socket.local_addr()?);

    loop {
        let mut buf = [0u8; 2048];  // A buffer to store the incoming data

        // Receive data from the socket
        let (amt, src) = socket.recv_from(&mut buf)?;

        // `amt` is the number of bytes received
        // `src` is the source address of the sender

        println!("Received {} bytes from {}", amt, src);

        // Handle the data (for now, just print it)
        let received_data = &buf[..amt];
        println!("Received data: {:?}", received_data);
    }
}