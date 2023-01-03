// use crate::crypto::field::FieldElement;
use super::{FieldElement, Secp256k1Param};
use num::bigint::BigInt;
use sha2::{Digest, Sha256};
use ripemd::{Ripemd160};




pub fn hash(msg: &str, n: Option<&BigInt>) -> FieldElement {
    // converting the message to bytes and then to a BigInt
    match n {
        Some(n) => {
            let mut hasher = Sha256::new();
            hasher.update(msg);
            let result = hasher.finalize();
            let mut bytes = [0; 32];
            bytes.copy_from_slice(&result[..]);
            let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
            FieldElement::new(num, n.clone())
        }
        None => {
            let params = Secp256k1Param::new();
            let mut hasher = Sha256::new();
            hasher.update(msg);
            let result = hasher.finalize();
            let mut bytes = [0; 32];
            bytes.copy_from_slice(&result[..]);
            let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
            FieldElement::new(num, params.prime())
        }
    }
}

// another function of hash , just that it takes a BigInt as input
pub fn hash_bigint(msg: BigInt) -> FieldElement {
    let params = Secp256k1Param::new();
    let mut hasher = Sha256::new();
    hasher.update(msg.to_string());
    let result = hasher.finalize();
    let mut bytes = [0; 32];
    bytes.copy_from_slice(&result[..]);
    let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
    FieldElement::new(num, params.prime())
}

// a hash function that takes a string slice and returns a string 

pub fn hash_str(msg: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(msg);
    let result = hasher.finalize();
    let mut bytes = [0; 32];
    bytes.copy_from_slice(&result[..]);
    let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
    num.to_str_radix(16)
}


pub fn hash_ripemd(string: &str) -> String {
    // 
    let mut hasher = Ripemd160::new();
    hasher.update(string);
    let result = hasher.finalize(); 
    let mut bytes = [0; 20];
    bytes.copy_from_slice(&result[..]);
    let num = BigInt::from_bytes_be(num::bigint::Sign::Plus, &bytes);
    num.to_str_radix(16)
}