mod crypto;

use crypto::secp256k1::Secp256k1Param;

use num::BigInt;

use crypto::hash::hash;
use crypto::keys::Keys;

use crate::crypto::{keys, FieldElement, get_rndm};
// use crypto::utils::{sign, verify};

use crypto::EllipticPoint;
use std::fs::File;
use std::io::Write;

fn main() {
    // generato

    // let prams = Secp256k1Param::new();

    // let prime = prams.prime().clone();

    // let e = hash("my secret key");
    // // println!("private key: {}", &e);
    // let msg_hash = hash("hello world");
    // println!("message hash: {}", msg_hash);
    // let k = FieldElement {
    //     num: BigInt::from(123),
    //     prime: prime.clone(),
    // };
    // let r = prams.generator().multiply(&k);

    // println!("r: {}", r);
    // let k_inv = k.num.modpow(&(prime.clone() - BigInt::from(2)), &prime);
    // println!("k_inv: {}", k_inv);
    // let s = (k_inv * (msg_hash + &e * r.x.num)) % &prime;
    // println!("s: {}", s);
    // let privat_key = FieldElement {
    //     num: e,
    //     prime: prime.clone(),
    // };

    // let public_key = prams.generator().multiply(&privat_key);
    // println!("public key: {}", public_key);
    // let private_key = Keys::new("my secret key".to_string());
    // println!("private key: {}", private_key.prk());
    // println!("public key: {}", private_key.pk());
    // // signing and verifying a message
    // let msg = "hello world";
    // let msg_hash = FieldElement {
    //     num: hash(msg),
    //     prime: Secp256k1Param::new().prime(),
    // };
    // let signature = private_key.sign(msg_hash.clone());
    // if private_key.verify(msg_hash, signature) {
    //     println!("signature is valid");
    // } else {
    //     println!("signature is invalid");
    // }

    /*
    Sure! Here is a pseudo code for signing a message using the Elliptic Curve Digital Signature Algorithm (ECDSA):
    1. Choose a random integer k in the range [1, n-1], where n is the order of the generator point on the elliptic curve.
    2. Calculate r = (k * G).x mod n, where G is the generator point and x is the x coordinate
    of the point.
    3. Calculate the hash of the message, denoted as h.
    4. Calculates = (k^-1 * (h + d * r)) mod n, where d is the private key and r is the value
    calculated in step 2.
    5. The signature is the pair (r, s).
    To verify the signature, the following steps can be followed:
    1. Calculate the hash of the message, denoted as h.
    2. Calculate w = s^-1 mod n.
    3. Calculate u1 = (h * w) mod n and u2 = (r * W) mod n.
    4. Calculate the point R = u1 * G + u2 * Q, where Q is the public key (a point on the elliptic
    curve).
    5. If R.x mod n is equal to r, the signature is valid. Otherwise, the signature is invalid.
    */

    let params = Secp256k1Param::new();

    let keys = Keys::new("my secret key".to_string());
    println!("private key: {}", keys.prk());
    println!("public key: {}", keys.pk());
    //
    let msg = "hello world";
    let msg_hash = hash(msg, None);
    println!("message hash: {}", msg_hash.num().to_str_radix(16));

    let signature = keys.sign(msg_hash.clone());
    println!("s {}, r {}", signature.s.num(), signature.s.num());
    //
    // // output signature to signature.sig file signature.r.x
    // // let mut file = File::create("signature.sig").unwrap();
    // // file.write_all(signature.r.num.to_str_radix(16).as_bytes())
    // //     .unwrap();
    //
    // // file.write_all(signature.s.num.to_str_radix(16).as_bytes())
    //
    // // if keys.verify(msg_hash, signature) {
    // //     println!("signature is valid");
    // // } else {
    // //     println!("signature is invalid");
    // // }

    // let params = Secp256k1Param::new();
    // let mut private = BigInt::from(1000);
    // private = private % params.n();
    // let public = params.generator().multiply(&FieldElement {
    //     num: private.clone(),
    //     prime: params.prime().clone(),
    // });
    // println!("private key: {}", private);
    // println!("public key: {}", public);
}
