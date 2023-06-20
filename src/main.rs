use chrono::Utc; //Used to get the current timestamp
use sha2::{Digest, Sha256}; //Used to hash the data

//Block structure for the blockchain
struct Block {
    timestamp: i64,
    data: String,
    hash: String,
    prev_hash: String,
}

//function for the struct to work with struct efficiently
impl Block {
    fn new_block(data: String, prev_hash: String) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(timestamp, &data, &prev_hash);

        Block {
            timestamp,
            data,
            hash,
            prev_hash,
        }
    }

    fn calculate_hash(timestamp: i64, data: &str, prev_hash: &str) -> String {
        //Here, format!() used to concat the string which are given as input
        let input = format!("{}{}{}", timestamp, data, prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let res = hasher.finalize();
        format!("{:x}", res)
    }
}

struct Blockchain {
    blocks: Vec<Block>, //Vec here is providing the dynamic storage, ultimately, the blocks will contain the dynamic collection of the Block instances.
                        //The vector provides methods to add, remove, and access elements efficiently. It automatically manages the memory allocation and resizing as elements are added or removed.
}

impl Blockchain {
    fn new_blockchain() -> Blockchain {
        let genesis_block = Block::new_block(
            "Genesis Block of vicky's first blockchain.".to_owned(), //.to_owned() are ensuring that the datatypes in new_block() function are passed as String and not &str.They're converting them
            "0".to_owned(),
        );

        Blockchain {
            blocks: vec![genesis_block], //This vec! actually initializing the blocks field by initializing single element i.e genesis block
        }
    }

    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
    }
}
fn main() {}
