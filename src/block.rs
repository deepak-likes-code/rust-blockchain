use super::*;
use bincode::{deserialize, serialize};
use colored::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::{debug, info};
use std::time::SystemTime;

const TARGET_HEX: usize = 4;
pub type Result<T> = std::result::Result<T, failure::Error>;

#[derive(Debug, Clone)]
pub struct Block {
    nonce: u32,
    height: usize,
    timestamp: u128,
    hash: String,
    prev_block_hash: String,
    transactions: String,
}

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn new_block(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp,
            nonce: 0,
            height,
            prev_block_hash,
            hash: String::new(),
            transactions: data,
        };

        block.proof_of_work()?;
        Ok(block)
    }

    fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block");
        while !self.validate()? {
            self.nonce += 1;
        }
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEX,
            self.nonce,
        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1 = vec![];
        vec1.resize(TARGET_HEX, '0' as u8);
        println!("{}", format!("{:#?}", vec1).green());

        Ok(&hasher.result_str()[0..TARGET_HEX] == String::from_utf8(vec1)?)
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::new_genesis_block()],
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev_block = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev_block.get_hash(), TARGET_HEX)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
        println!("{}", format!("{:#?}", b).green());

        dbg!(b);
    }
}
