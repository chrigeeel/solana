use std::{net::UdpSocket, thread::current};
use std::io;
use std::error::Error;

use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;


use {
    solana_ledger::shred::{
        max_entries_per_n_shred, max_ticks_per_n_shreds, ProcessShredsStats, ReedSolomonCache,
        Shred, ShredFlags, Shredder, DATA_SHREDS_PER_FEC_BLOCK, LEGACY_SHRED_DATA_CAPACITY,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    // Bind the socket to a local address
    let socket = UdpSocket::bind("0.0.0.0:8002")?;
    println!("Listening on {}", socket.local_addr()?);

    let client = RpcClient::new("https://proportionate-powerful-leaf.solana-mainnet.quiknode.pro/79c03ee439e0288092c46640d7cf521a1c598e19/");

    let current_slot = client.get_slot_with_commitment(CommitmentConfig{
        commitment: solana_sdk::commitment_config::CommitmentLevel::Processed,
    })?;
    let parsing_slot = current_slot + 10;

    let data_shreds: &mut Vec<Shred> = &mut [].into();

    loop {
        let mut buf = [0u8; 2048];  // A buffer to store the incoming data

        // Receive data from the socket
        let (amt, src) = socket.recv_from(&mut buf)?;

        // `amt` is the number of bytes received
        // `src` is the source address of the sender

        println!("Received {} bytes from {}", amt, src);

        // Handle the data (for now, just print it)
        let received_data = &buf[..amt];
        let shred = Shred::new_from_serialized_shred(received_data.to_vec())?;
        if !shred.is_data() {
            continue
        }

        println!("Parsed shred {:?} {:?}", shred.slot(), shred.is_data());

        if shred.slot() > parsing_slot + 10 {
            break;
        }

        if shred.is_data() && shred.slot() == parsing_slot {
            data_shreds.push(shred.clone())
        }


    }

    println!("Got shreds {:?}", data_shreds.len());

    Ok(())
}