// implementatation of the ECDSA signature algorithm
// r : x-coordinate of the point R
// s : k^-1 * (z + r * d) mod n
// z : hash of the message
use num::BigInt;

use super::EllipticPoint;
use crate::crypto::field::FieldElement;

// #[derive(Debug, Clone)]
pub struct Signature {
    pub r: EllipticPoint,
    pub s: BigInt,
}

impl Signature {
    pub fn new(r: EllipticPoint, s: BigInt) -> Signature {
        Signature { r, s }
    }
}
