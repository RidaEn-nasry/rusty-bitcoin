


use num::BigInt;
use num::bigint::Sign;

use crate::crypto::field::FieldElement;
use sha2::{Sha256, Digest};


pub fn hash(msg: &str, prime: BigInt) -> FieldElement {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    let num = BigInt::from_bytes_le(Sign::Plus, &result);
    FieldElement::new(num, prime)
}

