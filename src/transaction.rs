use super::*;
use std::collections::HashSet;

pub struct Output {
    pub to_addr: Address,
    pub amount: u64
}

impl Hashable for Output {
    fn bytes (&self) -> Hash {
        let mut bytes = vec![];

        bytes.extend(self.to_addr.bytes());
        bytes.extend(&u64_bytes(&self.amount));

        return bytes;
    }
}


pub struct Transaction{
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}
fn get_sum(outvecs: &Vec<Output>) -> u64{
    return outvecs
        .iter()
        .map(|x| x.amount) //Linq select 
        .sum();
}
fn get_hashes(outvecs: &Vec<Output>) -> HashSet<Hash>{
    return outvecs
    .iter()
    .map(|x| x.hash())
    .collect::<HashSet<Hash>>();
}
impl Transaction {
    //Get total amounts
    pub fn get_total_amount_of_inputs (&self) -> u64 {
        return get_sum(&self.inputs);
    }
    pub fn get_total_amount_of_outputs (&self) -> u64 {
        return get_sum(&self.outputs);
    }
    
    //Gets all hashes
    pub fn input_hashes(&self) -> HashSet<Hash> {
        return get_hashes(&self.inputs);
    } 
    pub fn output_hashes(&self) -> HashSet<Hash> {
        return get_hashes(&self.outputs);
    } 

    //Check if it is coinbase = there is no previous inputs
    pub fn is_coinbase(&self) -> bool {
        return self.inputs.len() == 0;
    }
}

impl Hashable for Transaction {
    fn bytes (&self) -> Hash {
        let mut bytes = vec![];
        bytes.extend(
            self.inputs
                .iter()
                .flat_map(|input| input.bytes()) //
                .collect::<Vec<u8>>()
        );
        bytes.extend(
            self.outputs
                .iter()
                .flat_map(|output| output.bytes()) //
                .collect::<Vec<u8>>()
        );
        
        return bytes;
    }
}