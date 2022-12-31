//  // parse_sec takes a compressed or uncompressed sec format public key and returns an EllipticPoint

use super::EllipticPoint;

use super::FieldElement;
use super::Secp256k1Param;
use hex::*;
use num::bigint::BigInt;
use num::bigint::Sign;

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
