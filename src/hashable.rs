use crypto_hashes::digest::Digest;
use crypto_hashes::sha3::Sha3_256;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha3_256::default();
        hasher.update(self.bytes());

        hasher.finalize().to_vec()
    }
}
