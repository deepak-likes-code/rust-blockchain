use std::collections::HashMap;

use log::info;

use crate::block::{Block, TARGET_HEX};
use crate::errors::Result;
use crate::transaction::{TXOutput, Transaction};

const GENESIS_COINBASE_DATA: &str =
    "The Times 03/Jan/2009 Chancellor on brink of second bailout for banks";

#[derive(Debug, Clone)]
pub struct Blockchain {
    db: sled::Db,
    current_hash: String,
}

pub struct BlockchainIter<'a> {
    current_hash: String,
    bc: &'a Blockchain,
}

impl Blockchain {
    //  pub fn new() -> Result<Blockchain> {
    //     let db = sled::open("data/blocks")?;
    //     match db.get("LAST")? {
    //         Some(hash) => {
    //             let lasthash = String::from_utf8(hash.to_vec())?;
    //             Ok(Blockchain {
    //                 db,
    //                 current_hash: lasthash,
    //             })
    //         }
    //         None => {
    //             let block = Block::new_genesis_block();
    //             db.insert(block.get_hash(), bincode::serialize(&block)?)?;
    //             db.insert("LAST", block.get_hash().as_bytes())?;
    //             let bc = Blockchain {
    //                 current_hash: block.get_hash(),
    //                 db,
    //             };
    //             bc.db.flush()?;
    //             Ok(bc)
    //         }
    //     }
    // }

    pub fn new() -> Result<Blockchain> {
        info!("open Blockchain");
        let db = sled::open("data/blocks")?;
        let hash = db
            .get("LAST")?
            .expect("Must create a blockchain database first");
        info!("Found the block database");
        let lasthash = String::from_utf8(hash.to_vec())?;
        Ok(Blockchain {
            db: db,
            current_hash: lasthash.clone(),
        })
    }

    // Creates a blockchain DB

    pub fn create_blockchain(address: String) -> Result<Blockchain> {
        info!("Creating new blockchain");
        let db = sled::open("data/blocks")?;
        info!("Creating new block database");
        let cbtx = Transaction::new_coinbase(address, String::from(GENESIS_COINBASE_DATA))?;
        let genesis = Block::new_genesis_block(cbtx);
        db.insert(genesis.get_hash(), bincode::serialize(&genesis)?)?;
        db.insert("LAST", genesis.get_hash().as_bytes())?;
        let bc = Blockchain {
            db,
            current_hash: genesis.get_hash(),
        };
        bc.db.flush()?;
        Ok(bc)
    }

    pub fn iter(&self) -> BlockchainIter {
        BlockchainIter {
            current_hash: self.current_hash.clone(),
            bc: &self,
        }
    }

    pub fn find_unspent_transaction(&self, address: &str) -> Vec<Transaction> {
        let mut spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut unspent_TXs: Vec<Transaction> = Vec::new();

        for block in self.iter() {
            for tx in block.get_transaction() {
                for index in 0..tx.vout.len() {
                    if let Some(ids) = spent_TXOs.get(&tx.id) {
                        if ids.contains(&(index as i32)) {
                            continue;
                        }
                    }

                    if tx.vout[index].can_be_unlock_with(address) {
                        unspent_TXs.push(tx.to_owned())
                    }
                }

                if !tx.is_coinbase() {
                    for i in &tx.vin {
                        if i.can_unlock_output_with(address) {
                            match spent_TXOs.get_mut(&i.txid) {
                                Some(v) => {
                                    v.push(i.vout);
                                }
                                None => {
                                    spent_TXOs.insert(i.txid.clone(), vec![i.vout]);
                                }
                            }
                        }
                    }
                }
            }
        }
        unspent_TXs
    }

    pub fn find_UTXO(&self, address: &str) -> Vec<TXOutput> {
        let mut utxos = Vec::<TXOutput>::new();
        let unspent_TXs = self.find_unspent_transaction(address);

        for tx in unspent_TXs {
            for out in &tx.vout {
                if out.can_be_unlock_with(address) {
                    utxos.push(out.clone())
                }
            }
        }

        utxos
    }

    pub fn find_spendable_outputs(
        &self,
        address: &str,
        amount: i32,
    ) -> (i32, HashMap<String, Vec<i32>>) {
        let mut unspent_outputs: HashMap<String, Vec<i32>> = HashMap::new();
        let mut accumulated = 0;
        let unspent_TXs = self.find_unspent_transaction(address);

        for tx in unspent_TXs {
            for index in 0..tx.vout.len() {
                if tx.vout[index].can_be_unlock_with(address) && accumulated < amount {
                    match unspent_outputs.get_mut(&tx.id) {
                        Some(v) => v.push(index as i32),
                        None => {
                            unspent_outputs.insert(tx.id.clone(), vec![index as i32]);
                        }
                    }
                    accumulated += tx.vout[index].value;

                    if accumulated >= amount {
                        return (accumulated, unspent_outputs);
                    }
                }
            }
        }
        (accumulated, unspent_outputs)
    }

    pub fn add_block(&mut self, transaction: Vec<Transaction>) -> Result<()> {
        let lasthash = self.db.get("LAST")?.unwrap();
        let new_block = Block::new_block(
            transaction,
            String::from_utf8(lasthash.to_vec())?,
            TARGET_HEX,
        )?;
        self.db
            .insert(new_block.get_hash(), bincode::serialize(&new_block)?)?;
        self.db.insert("LAST", new_block.get_hash().as_bytes())?;
        self.current_hash = new_block.get_hash();
        Ok(())
    }
}

impl<'a> Iterator for BlockchainIter<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(encode_block) = self.bc.db.get(&self.current_hash) {
            return match encode_block {
                Some(b) => {
                    if let Ok(block) = bincode::deserialize::<Block>(&b) {
                        self.current_hash = block.get_prev_hash();
                        Some(block)
                    } else {
                        None
                    }
                }
                None => None,
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new().unwrap();
        // b.add_block("data".to_string());
        // b.add_block("data2".to_string());
        // b.add_block("data3".to_string());

        for item in b.iter() {
            println!("{:?}", item)
        }

        dbg!(b);
    }
}
