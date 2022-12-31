use super::{FieldElement, Secp256k1Param, Signature};
use num::bigint::BigInt;
use num::bigint::Sign;

use rand::{thread_rng, Rng, RngCore};
// use num_bigint::bigint::BigInt;
// use num_bigint::ToBigInt;


pub fn get_rndm(n: &BigInt) -> BigInt {
    // generate a random number between 1 and n - 1
    let mut rng = thread_rng();
    let max = n.clone() - BigInt::from(1);
    let min = BigInt::from(1);
    let bytes = max.to_bytes_be();
    let mut num = BigInt::from_bytes_be(Sign::Plus, &rng.gen::<[u8; 32]>());
    while num > max {
        num = BigInt::from_bytes_be(Sign::Plus, &rng.gen::<[u8; 32]>());
    }
    num

    // let num: BigIn = rng.gen_range(min, max);t
}



// a function that takes a vector of raw bytes &[u8] 
//and returns a string of hex characters in big endian

// pub fn slice_to_hexb(bytes: &[u8]) -> String {
//     // convert the slice of raw bytes to a vector of hex characters
//     let mut hex = String::new();
//     for byte in bytes {
//         hex.push_str(&format!("{:02x}", byte));
//     }
//     hex
// }