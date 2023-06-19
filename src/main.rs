use chrono::Utc; //Used to get the current timestamp
use sha2::{Digest, Sha256}; //Used to hash the data

//Block structure for the blockchain
struct Block {
    timestamp: i64,
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn get_descrp(&self) {
        println!("Block: Timestamp:{}, data:{}", self.timestamp, self.data);
    }
}

fn main() {
    let new_block = Block {
        timestamp: 1234,
        data: String::from("This is the first block"),
        hash: String::from("0x213ead3bbbbaaaee2123"),
        prev_hash: String::from("0x23124214bbeaaffffff"),
    };

    new_block.get_descrp();
}
