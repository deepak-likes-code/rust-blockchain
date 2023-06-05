use crate::block::{self, Block};
use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::txn::TXOutputs;

// Represents the UTXOset

pub struct UTXOSet {
    pub blockchain: Blockchain,
}

impl UTXOSet {
    // Reindex rebuilds the UTXO set
    pub fn reindex(&self) -> Result<()> {
        std::fs::remove_dir("data/utxos")?;
        let db = sled::open("data/utxos")?;

        let utxos = self.blockchain.find_UTXO();

        for (txid, outs) in utxos {
            db.insert(txid.as_bytes(), bincode::serialize(&outs)?)?;
        }
        Ok(())
    }

    ///Updates the UTXO set with the transactions from the block
    ///
    /// The block is considered to be the tip of a blockchain

    pub fn update(&self, block: &Block) -> Result<()> {
        let db = sled::open("data/utxos")?;

        for tx in block.get_transaction() {
            if !tx.is_coinbase() {
                for vin in &tx.vin {
                    let mut update_outputs = TXOutputs {
                        outputs: Vec::new(),
                    };
                    let outs: TXOutputs = bincode::deserialize(&db.get(&vin.txid)?.unwrap());
                    for out_idx in 0..outs.outputs.len() {
                        if out_idx != vin.vout as usize {
                            update_outputs.outputs.push(outs.outputs[out_idx].clone());
                        }
                    }

                    if update_outputs.outputs.is_empty() {
                        db.remove(&vin.txid)?;
                    } else {
                        db.insert(vin.txid.as_bytes(), bincode::serialize(&update_outputs))
                    }
                }
            }

            let mut new_outputs = TXOutputs {
                outputs: Vec::new(),
            };
            for out in &tx.vout {
                new_outputs.outputs.push(out.clone());
            }
            db.insert(tx.id.as_bytes(), bincode::serialize(&new_outputs)?)?;
        }
        Ok(())
    }

    /// Count transactions returns the number of transactions in the UTXO set
    pub fn count_transactions(&self) -> Result<i32> {
        let mut counter = 0;
        let db = sled::open("data/utxo")?;
        for kv in db.iter() {
            kv?;
            counter += 1;
        }
        Ok(counter)
    }
}
