use crate::crypto::field::FieldElement;
use num::bigint::BigInt;
use sha2::{Digest, Sha256};

pub fn hash(msg: &str) -> BigInt {
    // converting the message to bytes and then to a BigInt
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    let mut bytes = [0; 32];
    bytes.copy_from_slice(&result[..]);
    let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
    num
}
