use std::ops::Rem;

use super::{FieldElement, Secp256k1Param, Signature};
use num::bigint::BigInt;
use num::bigint::Sign;
use num::ToPrimitive;

static BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

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

// a function that's encode a BigInt int a base58 string

pub fn encode_base58(num: &BigInt) -> String {
    // count how many leading zeros
    let mut leading_zeros = 0;
    for byte in num.to_bytes_be().1.iter() {
        if *byte == 0 {
            leading_zeros += 1;
        } else {
            break;
        }
    }
    //
    let mut num = num.clone();
    let mut result = String::new();
    while num > BigInt::from(0) {
        let new_num = &num / &BigInt::from(58);
        let rem = &num % &BigInt::from(58);

        num = new_num;

        result.push(BASE58_ALPHABET[rem.to_usize().unwrap()] as char);
    }
    for _ in 0..leading_zeros {
        result.push(BASE58_ALPHABET[0] as char);
    }
    result.chars().rev().collect()
}
