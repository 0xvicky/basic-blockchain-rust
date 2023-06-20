use chrono::Utc; //Used to get the current timestamp
use sha2::{Digest, Sha256}; //Used to hash the data
use std::io;

//Block structure for the blockchain
struct Block {
    timestamp: i64,
    data: String,
    hash: String,
    prev_hash: String,
    block_number: i64,
}

//function for the struct to work with struct efficiently
impl Block {
    fn new_block(data: String, prev_hash: String, block_number: i64) -> Block {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(timestamp, &data, &prev_hash, block_number);

        Block {
            timestamp,
            data,
            hash,
            prev_hash,
            block_number,
        }
    }

    fn calculate_hash(timestamp: i64, data: &str, prev_hash: &str, block_number: i64) -> String {
        //Here, format!() used to concat the string which are given as input
        let input = format!("{}{}{}{}", timestamp, data, prev_hash, block_number);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let res = hasher.finalize();
        format!("{:x}", res) //returns hash
    }
}

struct Blockchain {
    blocks: Vec<Block>, //Vec here is providing the dynamic storage, ultimately, the blocks will contain the dynamic collection of the Block instances.
                        //The vector provides methods to add, remove, and access elements efficiently. It automatically manages the memory allocation and resizing as elements are added or removed.
}

impl Blockchain {
    //Initiate new blockchain
    fn new_blockchain() -> Blockchain {
        let genesis_block = Block::new_block(
            "Genesis Block of vicky's first blockchain.".to_owned(), //.to_owned() are ensuring that the datatypes in new_block() function are passed as String and not &str.They're converting them
            "0".to_owned(),
            0,
        );

        Blockchain {
            blocks: vec![genesis_block], //This vec! actually initializing the blocks field by initializing single element i.e genesis block
        }
    }

    //Add new block to blockchain
    fn add_block(&mut self, data: String) {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new_block(
            data,
            prev_block.hash.clone(),
            prev_block.block_number.clone() + 1,
        );
        self.blocks.push(new_block);
    }
}
fn main() {
    let mut blockchain = Blockchain::new_blockchain();

    //Add new blockchain in runttime

    loop {
        let block_options = &blockchain.blocks.last();
        if let Some(block) = block_options {
            println!("");
            println!("Timestamp:{}", block.timestamp);
            println!("Data:{}", block.data);
            println!("Prev Hash:{}", block.prev_hash);
            println!("Hash:{}", block.hash);
            println!("Block Number:{}", block.block_number);
            println!("");
        } else {
            println!("No block found!")
        }

        println!("Enter the new transaction to the blockchain, or type 'quit' to exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input) //read the line of the input
            .expect("Failed to read the input");

        input = input.trim().to_owned();

        if input == "quit" {
            break;
        }
        blockchain.add_block(input);
        //Looping over blockchain to print all the blocks\
    }
}
