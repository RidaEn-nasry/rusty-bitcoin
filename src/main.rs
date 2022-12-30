mod crypto;

use crypto::secp256k1::Secp256k1Param;

use num::BigInt;

use crypto::ecdsa::ECDSA;
use crypto::hash::hash;
use signature::Signature;

use crate::crypto::{ecdsa, get_rndm, signature, FieldElement};
// use crypto::utils::{sign, verify};

use crypto::EllipticPoint;


fn main() {
    // generato

    // let prams = Secp256k1Param::new();

    // let prime = prams.prime().clone();

    // let e = hash("my secret key");
    // // println!("private key: {}", &e);
    // let z = hash("hello world");
    // println!("message hash: {}", z);
    // let k = FieldElement {
    //     num: BigInt::from(123),
    //     prime: prime.clone(),
    // };
    // let r = prams.generator().multiply(&k);

    // println!("r: {}", r);
    // let k_inv = k.num.modpow(&(prime.clone() - BigInt::from(2)), &prime);
    // println!("k_inv: {}", k_inv);
    // let s = (k_inv * (z + &e * r.x.num)) % &prime;
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
    // let z = FieldElement {
    //     num: hash(msg),
    //     prime: Secp256k1Param::new().prime(),
    // };
    // let signature = private_key.sign(z.clone());
    // if private_key.verify(z, signature) {
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
    let keys = ECDSA::new("my secret key".to_string());
    println!("private key: {}", keys.prk());
    println!("public key: {}", keys.pk());
    //
    let msg = "hello world";
    let z = hash(msg, None);
    // get random k
    let k = get_rndm(&params.n());
    // R = k * G
    let R = params.generator() * &k;
    let r = R.x.num.clone();
    let k_inv = k.modpow(&(params.n() - BigInt::from(2)), &params.n());
    let s = (k_inv * (z.num.clone() + r.clone() * keys.prk())) % &params.n();

    let public = params.generator() * &keys.prk();
    // println!("public key: {}", public.x.num);
    // // verify
    let s_inv = s.modpow(&(params.n() - BigInt::from(2)), &params.n());
    let u = z.num.clone() * s_inv.clone() % &params.n();
    let v = r.clone() * s_inv.clone() % &params.n();
    let R2 = &(params.generator() * &u) + &(public * &v);
    println!("R2: {}", R2.x.num);
    println!("R: {}", r);
    if R2.x.num == r {
        println!("signature is valid");
    } else {
        println!("signature is invalid");
    }
    
    // let u = z 
    // let h = z.num.clone();
    // let s1 = s.modpow(&(params.n() - BigInt::from(2)), &params.n());
    // let r2 = &(params.generator() * &(h.clone() * s1.clone())) + &(R * &(s1.clone() * keys.pk()));
    
    // println!("r2 {}", r2.x.num.to_string());
    // println!("r  {}", r.to_string());
    
    // if r2.x.num == r {
    //     println!("signature is valid");
    // } else {
    //     println!("signature is invalid");
    // }
    
    // let h = hash(&z.num.to_string(), None).num;





    // let signature = keys.sign(z.clone());
    // if keys.verify(z, signature) == true {
        // println!("signature is valid");
    // } else {
        // println!("signature is invalid");
    // }
    // println!("r {}", signature.r.x.num());
    // println!("s {}", signature.s.num());

    // signature.r.x = FieldElement {
    //     num: BigInt::parse_bytes(
    //         b"50239575108428408310413211650003525893286361243737924845510476823361742567724",
    //         10,
    //     )
    //     .unwrap(),
    //     prime: params.prime()
    // };
    // // println!("r {}", signature.r.x.num().to_str_radix(16));
    // signature.s = FieldElement {
    //     num: BigInt::parse_bytes(
    //         b"24148350574868926181878357970953993772966781636682646823600901823505844929932",
    //         10,
    //     )
    //     .unwrap(),
    //     prime: params.prime()
    // };
    // println!("s {}", signature.s.num().to_str_radix(16));
    // let signature =  Signature::new(
    //     EllipticPoint {
    //         x: FieldElement {
    //             num: BigInt::from(0x1c1b9b1b),
    //             prime: params.prime().clone(),
    //         },
    //         y: FieldElement {
    //             num: BigInt::from(0x1c1b9b1b),
    //             prime: params.prime().clone(),
    //         },
    //     },
    //     FieldElement {
    //         num: BigInt::from(0x1c1b9b1b),
    //         prime: params.prime().clone(),
    //     },
    // );
    // )
    // println!("s {}, r {}", signature.s.num(), signature.r.num());
    //
    // // output signature to signature.sig file signature.r.x
    // // let mut file = File::create("signature.sig").unwrap();
    // // file.write_all(signature.r.num.to_str_radix(16).as_bytes())
    // //     .unwrap();
    //
    // // file.write_all(signature.s.num.to_str_radix(16).as_bytes())
    //

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
