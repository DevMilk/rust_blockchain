use super::*;
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>//Current state of unspent outputs
}

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction
}
    
impl Blockchain {
    pub fn new() -> Self {
        return Blockchain {
            blocks: vec![],
            unspent_outputs: HashSet::new()
        }
    }
    pub fn update_with_block(&mut self, block: Block) -> Result<(),BlockValidationErr> {

        //Iterate through all blocks from start
        for (i, block) in self.blocks.iter().enumerate() {
            
            //Index mismatch control
            if block.index != i as u32 {return Err(BlockValidationErr::MismatchedIndex);}

            //Check if hash matches block's instance values
            if !block::check_difficulty(&block.hash(),block.difficulty) {return Err(BlockValidationErr::InvalidHash);}

            if block.index==0 {

                //Genesis block's previous block hash must equal to zero
                if block.prev_block_hash != vec![0; 32]  {return Err(BlockValidationErr::InvalidGenesisBlockFormat);}
            }
            else{
                let prev_block = &self.blocks[i-1];
                //Check timestamps
                if block.timestamp <= prev_block.timestamp {return Err(BlockValidationErr::AchronologicalTimestamp);}
                
                //Check previous block's hash is matching
                if block.prev_block_hash != prev_block.hash {return Err(BlockValidationErr::MismatchedPreviousHash);}
            }
        }

        //If there is at least one element in vector it will pop into this tuple else it is None
        if let Some((coinbase, transactions)) = block.transactions.split_first(){
            if !coinbase.is_coinbase(){
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            
            let mut total_fee = 0;
            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                //Can be optimized
                if !(&input_hashes - &self.unspent_outputs).is_empty() //Input hashes should contain only unspent outputs
                    ||
                    !(&input_hashes & &block_spent).is_empty() //Input hashes should NOT contain any outputs spent before
                {
                    return Err(BlockValidationErr::InvalidInput);
                }
                
                let input_value = transaction.get_total_amount_of_inputs();
                let output_value = transaction.get_total_amount_of_outputs();
                
                //it must be impossible for the output to be higher than the input
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                total_fee+= output_value - input_value;
                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }

            if coinbase.get_total_amount_of_outputs() < total_fee {return Err(BlockValidationErr::InvalidCoinbaseTransaction);}

            block_created.extend(coinbase.output_hashes());
            
            //Remove spent blocks
            self.unspent_outputs.retain(|output| 
                !block_spent.contains(output)); 
            self.unspent_outputs.extend(block_created);

        }

        self.blocks.push(block);
        return Ok(());
    }
}