use super::{Hash, Hashable};
use std::collections::HashSet;

//TODO: Fix bad name
#[derive(Clone)]
pub struct Details {
    //Stored as strings for now as not sure how best to store public keys
    pub payee: String,
    pub payer: String,
    pub amount: u64,
}

impl Hashable for Details {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.payee.as_bytes());
        bytes.extend(self.payer.as_bytes());
        bytes.extend(&self.amount.to_be_bytes());
        return bytes;
    }
}

pub struct Transaction {
    pub inputs: Vec<Details>,
    pub outputs: Vec<Details>,
}

impl Transaction {
    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.amount).sum()
    }
    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.amount).sum()
    }
    pub fn input_hashes(&self) -> HashSet<Hash> {
        self.inputs
            .iter()
            .map(|input| input.hash())
            .collect::<HashSet<Hash>>()
    }
    pub fn output_hashes(&self) -> HashSet<Hash> {
        self.outputs
            .iter()
            .map(|output| output.hash())
            .collect::<HashSet<Hash>>()
    }
    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(
            &self
                .inputs
                .iter()
                .flat_map(|input| input.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(
            &self
                .outputs
                .iter()
                .flat_map(|output| output.bytes())
                .collect::<Vec<u8>>(),
        );
        return bytes;
    }
}
