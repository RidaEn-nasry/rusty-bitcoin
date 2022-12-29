use std::fs::File;

use super::utils::get_rndm;
use super::EllipticPoint;
use crate::crypto::field::FieldElement;
use crate::crypto::hash::hash;
use crate::crypto::signature::Signature;
use crate::crypto::Secp256k1Param;
use num::bigint::BigInt;

pub struct Keys {
    secret_str: String,
    puk: BigInt,
    prk: BigInt,
}

impl Keys {
    pub fn new(secret: String) -> Keys {
        let params = Secp256k1Param::new();
        let secret = hash(&secret, None);

        let pk = params.generator().multiply(&FieldElement {
            num: secret.num(),
            prime: params.prime(),
        });
        Keys {
            secret_str: secret.num().to_string(),
            puk: pk.x.num(),
            prk: secret.num(),
        }
    }
    pub fn pk(&self) -> BigInt {
        self.puk.clone()
    }

    pub fn prk(&self) -> BigInt {
        self.prk.clone()
    }

    pub fn sign(&self, msg: FieldElement) -> Signature {
        let parms = Secp256k1Param::new();
        // let msg = hash(&msg);
        let k = BigInt::from(get_rndm(&parms.n()));
        let k = FieldElement {
            num: k,
            prime: parms.prime(),
        };

        // r = k * G
        let r = parms.generator().multiply(&k);
        // let r = parms.generator().multiply_bigint(&k);
        // k_inv = k^-1 = k^(n-2) mod n
        let k_inv = k
            .num()
            .modpow(&(parms.prime() - BigInt::from(2)), &parms.prime());
        let k_inv = FieldElement {
            num: k_inv,
            prime: parms.prime(),
        };
        // let k_inv = k
        //     .num()
        //     .modpow(&(parms.prime() - BigInt::from(2)), &parms.prime());
        // s = k^-1 * (z + r * d) mod n
        let s = &k_inv.num * (msg.num() + (&r.x.num() * self.prk()));
        let s = FieldElement::new(s, parms.prime());
        // let sign = Signature { r: r.x(), s };
        let sign = if s.num() > parms.n().clone() / BigInt::from(2) {
            Signature::new(
                r.x.clone(),
                FieldElement {
                    num: parms.n().clone() - &s.num,
                    prime: parms.prime().clone(),
                },
            )
        } else {
            Signature::new(
                r.x.clone(),
                FieldElement {
                    num: s.num(),
                    prime: parms.prime().clone(),
                },
            )
        };
        sign
    }
    // verify , z, signature
    pub fn verify(&self, z: FieldElement, signature: Signature) -> bool {
        // s^(n-2) mod n
        // get the inverse of s
        let s_inv = signature
            .s
            .num()
            .modpow(&(self.prk.clone() - BigInt::from(2)), &self.prk.clone());

        // let s_inv = signature.s.pow(&(self.prime.clone() - BigInt::from(2)));
        // u = z * s_inv mod n
        let u = z.num().modpow(&s_inv, &self.prk.clone());
        // let u = z.multiply(&s_inv);
        // v = r * s_inv mod n
        let v = signature.r.num.modpow(&s_inv, &self.prk.clone());
        // let v = signature.r.multiply(&s_inv);
        // u * G + v * P
        // let total = self.generator.multiply(&u).add(
        // &Secp256k1::new(signature.r.clone(), signature.s)
        // .point
        // .multiply(&v),
        // );
        let total = Secp256k1Param::new()
            .generator()
            .multiply(&FieldElement::new(u, Secp256k1Param::new().prime().clone()))
            .add(
                &Secp256k1Param::new()
                    .generator()
                    .multiply(&FieldElement::new(v, self.prk.clone())),
            );
        println!("total.x: {}", total.x.num);
        println!("signature.r: {}", signature.r.num);

        return total.x.eq(&signature.r);
    }
}
