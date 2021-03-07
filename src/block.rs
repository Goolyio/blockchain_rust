use super::{difficulty_bytes_as_u128, now, Hash, Hashable, Transaction};
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Hash: {}, TimeStamp: {}, Nonce: {}",
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.nonce
        )
    }
}

impl Block {
    //TODO: Change how this is done
    pub fn check_difficulty_with_hash(&self, hash: &Hash) -> bool {
        self.difficulty > difficulty_bytes_as_u128(&hash)
    }
    pub fn check_difficulty(&self) -> bool {
        self.difficulty > difficulty_bytes_as_u128(&self.hash)
    }
    pub fn new(prev_hash: Hash, transactions: Vec<Transaction>, difficulty: u128) -> Block {
        return Block {
            timestamp: now(),
            hash: vec![0; 32],
            prev_hash,
            nonce: 0,
            transactions,
            difficulty,
        };
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            //Probs shouldnt mutatate itself on every itteration
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if self.check_difficulty_with_hash(&hash) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(&self.hash);
        bytes.extend(&self.prev_hash);
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend(
            &self
                .transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&self.difficulty.to_be_bytes());
        return bytes;
    }
}
