mod crypto;
// mod crypto::elliptic_curve;
use crypto::elliptic_curve::EllipticPoint;
use crypto::field::FieldElement;
use crypto::hash::hash;
use crypto::secp256k1::Secp256k1;
use crypto::secp256k1::Secp256k1Param;

use num::BigInt;

fn main() {
    // generato

    let prams = Secp256k1Param::new();

    let prime = prams.prime().clone();

    let e = hash("my secret key");
    // println!("private key: {}", &e);
    let msg_hash = hash("hello world");
    println!("message hash: {}", msg_hash);
    let k = FieldElement {
        num: BigInt::from(123),
        prime: prime.clone(),
    };
    let r = prams.generator().multiply(&k);

    println!("r: {}", r);
    let k_inv = k.num.modpow(&(prime.clone() - BigInt::from(2)), &prime);
    println!("k_inv: {}", k_inv);
    let s = (k_inv * (msg_hash + &e * r.x.num)) % &prime;
    println!("s: {}", s);
    let privat_key = FieldElement {
        num: e,
        prime: prime.clone(),
    };

    let public_key = prams.generator().multiply(&privat_key);
    println!("public key: {}", public_key);
}
