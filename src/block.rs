use std::fmt::{ Formatter, Debug, Result };
use crate::Hashable;

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
        write!(f, "Block [{}], hash: {}, at: {} with: {}",
               &self.index,
               &hex::encode(&self.hash),
               &self.timestamp,
               &self.payload,
        )
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend(&*self.payload.as_bytes());

        bytes
    }
}
