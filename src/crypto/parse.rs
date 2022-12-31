//  // parse_sec takes a compressed or uncompressed sec format public key and returns an EllipticPoint

use super::EllipticPoint;

use super::FieldElement;
use super::Secp256k1Param;
use hex::*;
use num::bigint::BigInt;
use num::bigint::Sign;
use num::ToPrimitive;

use super::Signature;

// parsing the uncompressed sec format public key
pub fn parse_sec_u(sec_str: &str) -> EllipticPoint {
    let params = Secp256k1Param::new();
    // remove the first byte
    let sec = &sec_str[2..];
    // extract the x and y coordinates
    let x = &sec[0..64];
    let y = &sec[64..128];

    // decode the hex strings into slice of raw bytes &[u8]
    let x = match hex::decode(x) {
        Ok(x) => x,
        Err(e) => panic!("Error: {}", e),
    };
    let y = match hex::decode(y) {
        Ok(y) => y,
        Err(e) => panic!("Error: {}", e),
    };
    // convert the raw bytes (in big endian) to BigInt
    let x = BigInt::from_bytes_be(Sign::Plus, &x);
    let y = BigInt::from_bytes_be(Sign::Plus, &y);
    let x = FieldElement::new(x, params.prime());
    let y = FieldElement::new(y, params.prime());
    // create the EllipticPoint based in coordinates and the curve parameters
    EllipticPoint::new(x, y, params.a(), params.b())
}

// parsing the compressed sec format public key

pub fn parse_sec_c(sec: &str) -> EllipticPoint {
    // getting the parameters of the curve
    let params = Secp256k1Param::new();
    // extract the x coordinate
    let x = &sec[2..66];
    // changing string to slice of raw bytes
    let x = match hex::decode(x) {
        Ok(x) => x,
        Err(e) => panic!("Error: {}", e),
    };
    // converting the raw bytes (in big endian) to BigInt
    let x = BigInt::from_bytes_be(Sign::Plus, &x);
    // wrapping x in a FielElement to be able to perform calculatins on it
    let x = FieldElement::new(x.clone(), params.prime());
    // getting the y coordinate (y^2 = x^3 + b)
    let alpha = x.pow(&BigInt::from(3)) + &params.b();
    // square root of y
    let beta = match alpha.sqrt() {
        Some(y) => y,
        None => panic!("Error: y is not a square"),
    };

    // if the first byte is 02, the y coordinate is even, otherwise it is odd
    let is_even = &sec[0..2] == "02";
    // if the y coordinate is even, we use it as is, otherwise we use the negative of it
    let (even_beta, odd_beta) = if &beta.num % BigInt::from(2) == BigInt::from(0) {
        (
            beta.clone(),
            FieldElement::new(&params.prime() - &beta.num, params.prime()),
        )
    } else {
        (
            FieldElement::new(&params.prime() - &beta.num, params.prime()),
            beta.clone(),
        )
    };
    let y = if is_even { even_beta } else { odd_beta };
    EllipticPoint::new(x, y, params.a(), params.b())
}

pub fn parse_sec(sec: &str) -> EllipticPoint {
    if &sec[0..2] == "04" {
        // uncompressed sec format
        parse_sec_u(sec)
    } else {
        // compressed sec format
        parse_sec_c(sec)
    }
}

// convert a hex string to a BigInt
pub fn hex_to_bigint(hex_str: &str) -> Option<BigInt> {
    // if the hex string is not even, add a 0 at the beginning
    let hex_str = match hex::decode(hex_str) {
        Ok(hex_vec) => BigInt::from_bytes_be(Sign::Plus, &hex_vec),
        Err(e) => {
            println!("Error: {}", e);
            return None;
        }
    };
    Some(hex_str)
}

// parsing signature in DER format

// fn extract_r_s(sig: &str) -> (String, String) {
//     // remove the first byte
//     let sig = &
// }
pub fn parse_der(signature: &str) -> Result<Signature, String> {
    // if signature is not in DER format return an error
    if &signature[0..2] != "30" {
        return Err("Error: signature is not in DER format".to_string());
    }
    // println!("*********");
    // remove the first byte
    let sig = &signature[2..];
    // println!("sig: {}", sig);
    // get length of all
    // let len_all_str = match hex_to_bigint(&sig[0..2]) {
    //     Some(len) => len,
    //     None => return Err("Error: signature is not in DER format".to_string()),
    // };
    // remove the first byte
    let sig = &sig[4..];

    // get length of r
    let mut len_r = match hex_to_bigint(&sig[0..2]) {
        Some(len) => len,
        None => return Err("Error: signature is not in DER format".to_string()),
    };
    println!("len_r: {}", len_r);
    // remove the length byte
    let sig = &sig[2..];
    // let sig = match len_r.to_usize().unwrap() % 2 == 0{
    //     true => sig.to_string(),
    //     false => {
    //         len_r = len_r + BigInt::from(1);
    //         let mut sig = sig.to_string();
    //         sig.insert(0, '0');
    //         sig
    //     }
    // };
    // 000a685596cfe49ef3bb92c3e7c166bf8df172bdbaa902cee66b72e2d1ec064456702204d71d86b3901adae90ae4ff7c6062670ca07cf612ffe5666675300b6a76ed49e
    // 000a685596cfe49ef3bb92c3e7c166bf8df172bdbaa902cee66b72e2d1ec0644567
    // println!("sig3: {}", sig);

    // get r
    let r_str = &sig[0..len_r.to_usize().unwrap() * 2];
    println!("r_str: {}", r_str);
    let r = match hex_to_bigint(r_str) {
        Some(r) => r,
        None => return Err("Error: signature is not in DER format".to_string()),
    };
    println!("r: {}", r);

    println!("*******************");
    // remove r bytes
    let sig = &sig[len_r.to_usize().unwrap() * 2..];

    println!("sig4: {}", sig);
    let sig = &sig[2..];
    println!("len in str: {}", &sig[0..2]);
    // get length of s
    let mut s_len = match hex_to_bigint(&sig[0..2]) {
        Some(s) => s,
        None => return Err("Error: signature is not in DER format".to_string()),
    };

    println!("s_len: {}", s_len);

    // remove the first byte
    let sig = &sig[2..];
    // if s_len is not even, add a 0 at the beginning of sig
    let sig = match s_len.to_usize().unwrap() % 2 != 0 {
        true => {
            let mut sig = sig.to_string();
            sig.insert(0, '0');
            s_len += 1;
            sig
        }
        false => sig.to_string(),
    };

    // get s
    let s_str = &sig[..];
    println!("s_str: {}", s_str);
    let s = match hex_to_bigint(s_str) {
        Some(s) => s,
        None => return Err("Error: signature is not in DER format".to_string()),
    };
    println!("s: {}", s);

    Ok(Signature::new(r, s))
}
