use std::fmt::{ Formatter, Debug, Result };
use Hashable;

type BlockHash = Vec<u8>;

pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
}

impl Block {
    pub fn new(index: u64, timestamp: u128, prev_block_hash: BlockHash, nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0; 32],
            nonce,
            payload
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Block [{}]:  at: {} with: {}",
               &self.index,
               // &hex::encode(&self.hash),
               &self.timestamp,
               &self.payload,
        )
    }
}

impl Hashable for Block {

}