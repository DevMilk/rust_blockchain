//!!!Algorithm of this project is not the safest and efficient way. It is just for training purposes to understand how blockchain works.  

use blockchainlib::*;
fn main(){
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut genesis = Block::new(0,now(),vec![0; 32], vec![
        Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output{
                    to_addr: "Alice".to_owned(),
                    amount: 50,
                },
                transaction::Output{
                    to_addr: "Bob".to_owned(),
                    amount: 7,
                }
            ]
        }
    ], difficulty);

    genesis.hash = genesis.hash();
    
    println!("{:?}", &genesis);

    genesis.mine();
    println!("{:?}", &genesis);

    let mut prev_hash = genesis.hash().clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis).expect("Failed to add genesis block");

}