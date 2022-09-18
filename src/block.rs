use std::fmt::{self, Debug, Formatter}; //Libraries for Traits
use super::*; //import all modules on same folder 

//Block struct
pub struct Block{
    pub index: u32, 
    pub timestamp: u128,
    pub hash: Hash, // Hash = Vec<u8> defined on lib.rs
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128
}

//Implementation similiar to ToString in Java/C# etc.
impl Debug for Block{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at : {} with : {}, nonce: {}",
            &self.index, 
            &hex::encode(&self.hash), 
            &self.timestamp, 
            &self.transactions.len(),
            &self.nonce)
    }
}
//Implementations for block
impl Block {

    //Constructor
    pub fn new (
        index: u32, 
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128
    ) -> Self {
        return Block{
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty
        }
    }

    pub fn mine(&mut self){
        //Find such a nonce value that block's hash starts with n 0s (n=difficulity)
        while self.nonce <=u64::max_value() && check_difficulty(&self.hash(), self.difficulty) == false {
            self.nonce += 1;
        }
        self.hash = self.hash();
    }
}

//Implementation with hashable trait
impl Hashable for Block {
    fn bytes(&self) -> Hash {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|x| x.bytes())
                .collect::<Vec<u8>>()
        );
        bytes.extend(&u128_bytes(&self.difficulty));
        return bytes;
    }
}

pub fn check_difficulty(hash: &Hash, difficulty:u128)
-> bool{
    return difficulty > difficulty_bytes_as_u128(&hash)
}