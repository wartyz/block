// Video 3    5:22
use blockchainlib::*;

fn main() {
    let difficulty = 0x0000ffff_ffffffff_ffffffff_ffffffff;
    let mut block = Block::new(
        0,
        0,
        vec![0; 32],
        118318,
        "Genesis block!".to_owned(),
        difficulty);

//    block.hash=block.hash();
//
//    println!("{:?}",&block);

    block.mine();

    let mut last_hash = block.hash.clone();

    println!("Mined genesis block {:?}", &block);

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            0,
            last_hash,
            118318,
            "Another block".to_owned(),
            difficulty);


        block.mine();

        println!("Mined genesis block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
    }
}
