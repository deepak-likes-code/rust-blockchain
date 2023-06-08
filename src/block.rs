use super::*;
use crate::errors::Result;
use crate::transaction::Transaction;
use bincode::{deserialize, serialize};
use colored::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::{debug, info};
use merkle_cbt::merkle_tree::{Merge, CBMT};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub const TARGET_HEX: usize = 4;

struct MergeTX {}

impl Merge for MergeTX {
    type Item = Vec<u8>;

    fn merge(left: &Self::Item, right: &Self::Item) -> Self::Item {
        let mut hasher = Sha256::new();
        let mut data = left.clone();
        data.append(&mut right.clone());
        hasher.input(&data);
        let mut re: [u8; 32] = [0; 32];
        hasher.result(&mut re);
        re.to_vec()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    nonce: u32,
    height: usize,
    timestamp: u128,
    hash: String,
    prev_block_hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new_block(
        data: Vec<Transaction>,
        prev_block_hash: String,
        height: usize,
    ) -> Result<Block> {
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

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }

    fn hash_transactions(&self) -> Result<Vec<u8>> {
        let mut transactions = Vec::new();
        for tx in &self.transactions {
            transactions.push(tx.clone().hash()?.as_bytes().to_owned());
        }
        let merkle_tree = CBMT::<Vec<u8>, MergeTX>::build_merkle_tree(&*transactions);
        Ok(merkle_tree.root())
    }

    pub fn get_prev_hash(&self) -> String {
        self.prev_block_hash.clone()
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
            self.hash_transactions()?,
            self.timestamp,
            TARGET_HEX,
            self.nonce,
        );

        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    pub fn new_genesis_block(coinbase: Transaction) -> Block {
        Block::new_block(vec![coinbase], String::new(), 0).unwrap()
    }

    pub fn get_transaction(&self) -> &Vec<Transaction> {
        &self.transactions
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
