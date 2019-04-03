// 22:39
use blockchainlib::*;

fn main() {
    let mut block=Block::new(
        0,
        0,
        vec![0;32],
        118318,
        "Genesis block!".to_owned(),
        0x0000ffff_ffffffff_ffffffff_ffffffff);

    block.hash=block.hash();

    println!("{:?}",&block);

    //block.mine();

    //println!("{:?}",&block);
}
