use std::io;
use std::fmt::Formatter;
use std::fmt::Debug;
use std::fmt::Result;

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

fn main() {
	let block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
	println!("{:?}", &block);
}