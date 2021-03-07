use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct BlockChain {
    pub blocks: Vec<Block>,
    unspent_details: HashSet<Hash>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![],
            unspent_details: HashSet::new(),
        }
    }
    pub fn last_block(&self) -> Option<&Block> {
        let length = self.blocks.len();
        if length != 0 {
            return Some(&self.blocks[length - 1]);
        } else {
            return None;
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        if self.validate_block(&block) {
            if let Some((coinbase, transactions)) = block.transactions.split_first() {
                // if !coinbase.is_coinbase() {
                //     return Err(BlockValidationErr::InvalidCoinbaseTransaction);
                // }
                let mut block_spent: HashSet<Hash> = HashSet::new();
                let mut block_created: HashSet<Hash> = HashSet::new();
                let mut total_fee = 0;
                for transaction in transactions {
                    let input_hashes = transaction.input_hashes();

                    if !(&input_hashes - &self.unspent_details).is_empty()
                        || !(&input_hashes & &block_spent).is_empty()
                    {
                        return Err(BlockValidationErr::InvalidInput);
                    }
                    let input_value = transaction.input_value();
                    let output_value = transaction.output_value();
                    if output_value > input_value {
                        return Err(BlockValidationErr::InsufficientInputValue);
                    }
                    let fee = input_value - output_value;
                    total_fee += fee;
                    block_spent.extend(input_hashes);
                    block_created.extend(transaction.output_hashes());
                }
                if coinbase.output_value() < total_fee {
                    return Err(BlockValidationErr::InvalidCoinbaseTransaction);
                } else {
                    block_created.extend(coinbase.output_hashes());
                }

                self.unspent_details
                    .retain(|output| !block_spent.contains(output));
                self.unspent_details.extend(block_created);
            }
            self.blocks.push(block);

            Ok(())
        } else {
            return Err(BlockValidationErr::AchronologicalTimestamp);
        }
    }
    pub fn validate_block(&self, block: &Block) -> bool {
        if !block.check_difficulty() {
            return false;
        }
        //Not gen block
        if self.blocks.len() != 0 {
            let prev_block = self.last_block().expect("No previous block found");
            if prev_block.hash != block.prev_hash {
                return false;
            }
        } else {
            if block.prev_hash != vec![0; 32] {
                return false;
            }
        }
        return true;
    }
}
