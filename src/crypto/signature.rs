

// implementatation of the ECDSA signature algorithm
// r : x-coordinate of the point R
// s : k^-1 * (z + r * d) mod n
// z : hash of the message


use crate::crypto::field::FieldElement;

pub struct Signature {
    pub r: FieldElement,
    pub s: FieldElement,
}


impl Signature {
    pub fn new(r: FieldElement, s: FieldElement) -> Signature {
        Signature { r, s }
    }
}

