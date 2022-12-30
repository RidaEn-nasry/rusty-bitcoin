use std::fs::File;
use std::ops::{Mul, Sub};

use super::utils::get_rndm;
use super::{hash_bigint, EllipticPoint};
use crate::crypto::field::FieldElement;
use crate::crypto::hash::hash;
use crate::crypto::signature::Signature;
use crate::crypto::Secp256k1Param;
use num::bigint::BigInt;
use sha2::digest::typenum::private::BitDiff;

pub struct ECDSA {
    secret_str: String,
    puk: EllipticPoint,
    prk: BigInt,
}

impl ECDSA {
    pub fn new(secret: String) -> ECDSA {
        let params = Secp256k1Param::new();
        let prv = hash(&secret, None);
        let pk = params.generator() * &prv;  

        // let pk = params.generator().multiply(&FieldElement {
        //     num: secret.num(),
        //     prime: params.prime(),
        // });
        ECDSA {
            secret_str: secret,
            puk: pk,
            prk: prv.num,
        }
    }
    pub fn pk(&self) -> EllipticPoint {
        self.puk.clone()
    }

    pub fn prk(&self) -> BigInt {
        self.prk.clone()
    }

    pub fn sign(&self, z: FieldElement) -> Signature {
        // this struct contains all secp256k1 parameters
        let params = Secp256k1Param::new();
        // get a random 1 < k < n - 1
        let k = BigInt::from(get_rndm(&params.n()));
        // k % prime
        // let k = FieldElement {
        //     num: k,
        //     prime: params.prime(),
        // };

        // R = k * G
        let R = params.generator() * &k;
        // get r.x
        let r = R.x.num.clone();
        // k^-1 = k^(n-2) mod n
        let k_inv = k.modpow(&(params.n() - BigInt::from(2)), &params.n());
        // s = k^-1 * (z + r * d) mod n
        let s = (k_inv * (z.num.clone() + r.clone() * self.prk())) % &params.n();

        Signature { r: R, s }
    }
    pub fn verify(&self, z: FieldElement, signature: Signature) -> bool {
        let params = Secp256k1Param::new();
        // re-calculate the hash
        // let h = hash(&z.num.to_string(), None).num;
        // s^-1 = s^(n-2) mod n
        // s^-1 = s^(n-2) mod n
        let s_inv = signature.s.modpow(&(params.n() - BigInt::from(2)), &params.n());

        // u = z * s^-1 mod n
        let u = z.num.clone() * s_inv.clone() % &params.n();
        let v = signature.r.x.num.clone() * &s_inv % &params.n();
        let r2 = &(params.generator() * &u) + &(self.pk() * &v);

        println!("r2.x.num: {:?}", r2.x.num);
        println!("signature.r.x.num: {:?}", signature.r.x.num);
        &r2.x.num == &signature.r.x.num

        // let s1 = signature
        //     .s
        //     .modpow(&(params.n() - BigInt::from(2)), &params.n());
        // // r2 = (h * s1) * G + (r * s1) * pubkey
        // let r2 = &(params.generator() * &(h * &s1)) + &(signature.r.clone() * &(self.pk() * &s1));
        // // check if r1 == r2
        // &r2 == &signature.r
    }
}
