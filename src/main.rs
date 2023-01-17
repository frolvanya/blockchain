use chrono::{Local, Utc};
use sha2::{Digest, Sha256};

use log::{info, warn};
use std::io::Write;

const DIFFICULTY_PREFIX: &str = "00000";

struct Blockchain {
    blocks: Vec<Block>,
}

struct Block {
    id: u64,
    hash: String,
    previous_hash: String,
    timestamp: i64,
    data: String,
    nonce: u64,
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Self::mine(id, previous_hash.clone(), timestamp, data.clone());

        Self {
            id,
            hash,
            previous_hash,
            timestamp,
            data,
            nonce,
        }
    }

    fn hash(id: u64, previous_hash: String, timestamp: i64, data: String, nonce: u64) -> String {
        let unified_block_data = format!("{}{}{}{}{}", id, previous_hash, timestamp, data, nonce);

        let mut hasher = Sha256::new();
        hasher.update(unified_block_data);
        format!("{:x}", hasher.finalize())
    }

    fn mine(id: u64, previous_hash: String, timestamp: i64, data: String) -> (String, u64) {
        let mut nonce = 0;

        loop {
            let hash = Self::hash(id, previous_hash.clone(), timestamp, data.clone(), nonce);

            if hash.as_str().starts_with(DIFFICULTY_PREFIX) {
                info!("Block #{} was successfully mined", id);
                return (hash, nonce);
            }

            nonce += 1;
        }
    }
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: Vec::new() }
    }

    fn create_genesis(&mut self) {
        let timestamp = Utc::now().timestamp();
        let (hash, nonce) = Block::mine(
            0,
            String::from("genesis"),
            timestamp,
            String::from("genesis"),
        );

        let genesis_block = Block {
            id: 0,
            hash,
            previous_hash: String::from("genesis"),
            timestamp,
            data: String::from("genesis"),
            nonce,
        };

        self.blocks.push(genesis_block);
        info!("Genesis block was successfully created and added to the blockchain");
    }

    fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
        if (block.id == previous_block.id + 1)
            && block.hash.starts_with(DIFFICULTY_PREFIX)
            && (block.previous_hash == previous_block.hash)
            && (Block::hash(
                block.id,
                block.previous_hash.clone(),
                block.timestamp,
                block.data.clone(),
                block.nonce,
            ) == block.hash)
        {
            info!("Block #{} is valid", block.id);
            return true;
        }

        warn!("Block #{} is invalid", block.id);
        false
    }

    fn is_chain_valid(&self) -> bool {
        for block_index in 1..self.blocks.len() {
            if !self.is_block_valid(&self.blocks[block_index], &self.blocks[block_index - 1]) {
                warn!("Blockchain is invalid");
                return false;
            }
        }

        info!("Blockchain is valid");
        true
    }

    fn try_add_block(&mut self, block: Block) {
        let previous_block = self
            .blocks
            .last()
            .expect("should be at least one block in the blockchain");

        if self.is_block_valid(&block, previous_block) {
            self.blocks.push(block);
            info!("Block was successfully added to the blockchain");
        } else {
            warn!(
                "Block is invalid, cannot push block #{} to the blockchain",
                block.id
            );
        }
    }
}

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, log::LevelFilter::Info)
        .init();

    let mut blockchain = Blockchain::new();
    blockchain.create_genesis();

    loop {
        let previous_block = blockchain
            .blocks
            .last()
            .expect("should be at least one block in the blockchain");
        let new_block = Block::new(
            previous_block.id + 1,
            previous_block.hash.clone(),
            String::from("Hello"),
        );

        blockchain.try_add_block(new_block);

        if blockchain.blocks.len() % 10 == 0 {
            blockchain.is_chain_valid();
        }
    }
}
