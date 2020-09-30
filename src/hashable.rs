pub trait Hashable {
    // fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> &str {
        "Hashing test"
        // crypto_hash::digest(crypto_hash::Alghorith::SHA256, &self.bytes());
    }
}