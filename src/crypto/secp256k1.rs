
use crate::crypto::elliptic_curve::Point;
use num::bigint::BigInt;
// the prime and the generator point of the secp256k1 curve
pub const PRIME : BigInt = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap();
pub const G : Point = Point::new(
    FieldElement::new(BigInt::parse_bytes(b"55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap(), PRIME.clone()),
    FieldElement::new(BigInt::parse_bytes(b"32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap(), PRIME.clone()),
    FieldElement::new(BigInt::from(0), PRIME.clone()),
    FieldElement::new(BigInt::from(7), PRIME.clone()),
);

// implementing the secp256k1 curve, which is just a wrapper around the Point struct
pub struct Secp256k1 {
    pub point: Point,
}

impl Secp256k1 {
    pub fn new(x: FieldElement, y: FieldElement) -> Secp256k1 {
        let a = FieldElement::new(BigInt::from(0), PRIME.clone());
        let b = FieldElement::new(BigInt::from(7), PRIME.clone());
        let p = Point::new(x, y, a, b);
        Secp256k1 { point: p }
    }
}



