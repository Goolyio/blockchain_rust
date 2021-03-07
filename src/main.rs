use blockchainlib::*;

fn main() {
    let difficulty: u128 = 0x000fffffffffffffffffffffffffffff;
    let mut genisis_block = Block::new(
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![transaction::Details {
                payee: "Kieran".to_owned(),
                payer: "".to_owned(),
                amount: 300,
            }],
        }],
        difficulty,
    );
    genisis_block.mine();

    println!("Mined genesis block {:?}", &genisis_block);
    let mut last_hash = genisis_block.hash.clone();
    let mut block_chain = BlockChain::new();

    block_chain
        .update_with_block(genisis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        last_hash,
        vec![Transaction {
            inputs: vec![block_chain.blocks[0].transactions[0].outputs[0].clone()],
            outputs: vec![transaction::Details {
                payee: "Tim".to_owned(),
                payer: "Kieran".to_owned(),
                amount: 200,
            }],
        }],
        difficulty,
    );
    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    block_chain
        .update_with_block(block)
        .expect("Failed to add block");
}
