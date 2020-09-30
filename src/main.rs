mod block;
pub use crate::block::Block;
mod hashable;
pub use crate::hashable::Hashable;

fn main() {
	let block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
	println!("{:?}", &block);

	println!("{:?}", block.hash());
}