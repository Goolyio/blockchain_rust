use super::Hash;
use crypto_hash::Algorithm;

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Hash {
        crypto_hash::digest(Algorithm::SHA256, &self.bytes())
    }
}
