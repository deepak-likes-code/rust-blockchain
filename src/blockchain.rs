use crate::block::{Block, TARGET_HEX};
use crate::errors::Result;
use colored::*;

#[derive(Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>,
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
