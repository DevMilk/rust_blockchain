//!!!Algorithm of this project is not the safest and efficient way. It is just for training purposes to understand how blockchain works.  

use blockchainlib::*;
fn main(){

    //Set difficulity
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    //Initialize a genesis block (index 0)
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

    //Generate hash of genesis block
    genesis.hash = genesis.hash();
    
    println!("{:?}", &genesis);

    //Mine to find right nonce value to match the difficulity(number of starting zeros)
    genesis.mine();
    println!("{:?}", &genesis);

    //Add new block after genesis block and update it.
    let mut prev_hash = genesis.hash().clone();
    let mut blockchain = Blockchain::new();
    blockchain.update_with_block(genesis).expect("Failed to add genesis block");

}