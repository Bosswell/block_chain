mod block;
mod hashable;

pub use crate::block::Block;
pub use crate::hashable::Hashable;

fn main() {
	let mut block = Block::new(0, 0, vec![0; 32], 0, "Genesis block".to_owned());
	// println!("{:?}", &block);

    let hash = block.hash();
	println!("{:?}", hash);

    block.hash = hash;
    println!("{:?}", &block);
}
