use std::result;
use super::hashing::*;
// use num::bigint::BigInt;
extern crate num;
use num::bigint::BigInt;
// use num::bigint



// use num::bigint::Sign;
use num::{ToPrimitive, Num};

static BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";



pub fn encode_base58check(sec: &str) -> String {
    
    let sec_sha256 = hash_str(sec);
    let mut sec_rmd160 = hash_ripemd(sec_sha256.as_str());
    sec_rmd160.insert_str(0, "0"); 
    // add the checksum 
    let checksum = hash_str(sec_rmd160.as_str());
    // let checksum = hash_str(checksum.as_str());
    let checksum = checksum.chars().take(8).collect::<String>();
    sec_rmd160.push_str(checksum.as_str());
    // converting to base58
    let mut num = BigInt::parse_bytes(sec_rmd160.as_bytes(), 16).unwrap();
    let mut result = Vec::new();
    while num > BigInt::from(0) {
        let (new_num, rem) = (&num.clone() / &BigInt::from(58), &num % &BigInt::from(58));
        result.push(BASE58_ALPHABET[rem.to_usize().unwrap()]);
        num = new_num;
    }
    result.reverse();
    let mut result = String::from_utf8(result).unwrap();
    // add the leading 1s
    let mut leading_ones = 0;
    for c in sec_rmd160.chars() {
        if c == '0' {
            leading_ones += 1;
        } else {
            break;
        }
    }
    for _ in 0..leading_ones {
        result.insert(0, '1');
    }
    result  
}
