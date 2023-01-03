mod crypto;
// mod crypto::elliptic_curve;
use crypto::elliptic_curve::Point;
use crypto::field::FieldElement;
use num::BigInt;
use crypto::hash::hash;


fn main() {
    // generato
    let hash = hash("hello", BigInt::from(2));
    println!("hash: {}", hash.num());

}
