use crate::crypto::elliptic_curve::EllipticPoint;
use crate::crypto::field::FieldElement;
use crate::crypto::Signature;
use num::bigint::BigInt;

// the prime and the generator point of the secp256k1 curve
// pub const PRIME : BigInt = BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap();

// pub const PRIME : &BigInt = &BigInt::parse_bytes(b"115792089237316195423570985008687907853269984665640564039457584007908834671663", 10).unwrap();

// pub const G : Point = Point::new(
//     FieldElement::new(BigInt::parse_bytes(b"55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap(), ),
//     FieldElement::new(BigInt::parse_bytes(b"32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap(), ),
//     FieldElement::new(BigInt::from(0), ),
//     FieldElement::new(BigInt::from(7), PRIME),
// );
// implementing the secp256k1 curve, which is just a wrapper around the Point struct

pub struct Secp256k1Param {
    pub prime: BigInt,
    pub a: FieldElement,
    pub b: FieldElement,
    pub generator: EllipticPoint,
}

impl Secp256k1Param {
    pub fn new() -> Secp256k1Param {
        let prime = BigInt::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
            10,
        )
        .unwrap();
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let generator = EllipticPoint::new(
            FieldElement::new(BigInt::parse_bytes(b"55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap(), prime.clone()),
            FieldElement::new(BigInt::parse_bytes(b"32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap(), prime.clone()),
            a.clone(),
            b.clone(),
        );

        Secp256k1Param {
            prime,
            a,
            b,
            generator,
        }
    }

    pub fn prime(&self) -> BigInt {
        self.prime.clone()
    }
    pub fn a(&self) -> FieldElement {
        self.a.clone()
    }

    pub fn b(&self) -> FieldElement {
        self.b.clone()
    }

    pub fn generator(&self) -> EllipticPoint {
        self.generator.clone()
    }
}

pub struct Secp256k1 {
    pub prime: BigInt,
    pub generator: EllipticPoint,
    pub point: EllipticPoint,
}

impl Secp256k1 {
    pub fn new(x: FieldElement, y: FieldElement) -> Secp256k1 {
        let prime = BigInt::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
            10,
        )
        .unwrap();
        let a = FieldElement::new(BigInt::from(0), prime.clone());
        let b = FieldElement::new(BigInt::from(7), prime.clone());
        let generator = EllipticPoint::new(
            FieldElement::new(BigInt::parse_bytes(b"55066263022277343669578718895168534326250603453777594175500187360389116729240", 10).unwrap(), prime.clone()),
            FieldElement::new(BigInt::parse_bytes(b"32670510020758816978083085130507043184471273380659243275938904335757337482424", 10).unwrap(), prime.clone()),
            a.clone(),
            b.clone(),
        );

        let point = EllipticPoint::new(x, y, a, b);
        Secp256k1 {
            prime,
            generator,
            point,
        }
    }

    // verify , z, signature
    pub fn verify(&self, z: FieldElement, signature: Signature) -> bool {
        // s^(n-2) mod n
        // get the inverse of s
        let s_inv = signature.s.pow(&(self.prime.clone() - BigInt::from(2)));
        // u = z * s_inv mod n
        let u = z.multiply(&s_inv);
        // v = r * s_inv mod n
        let v = signature.r.multiply(&s_inv);
        // u * G + v * P
        let total = self.generator.multiply(&u).add(
            &Secp256k1::new(signature.r.clone(), signature.s)
                .point
                .multiply(&v),
        );
        return total.x.eq(&signature.r);
    }
}


