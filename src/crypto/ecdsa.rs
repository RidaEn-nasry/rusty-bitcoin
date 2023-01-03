use super::utils::get_rndm;
use super::EllipticPoint;
use crate::crypto::field::FieldElement;
use super::hashing::*;
use crate::crypto::signature::Signature;
use crate::crypto::Secp256k1Param;
use num::bigint::BigInt;

pub struct ECDSA {
    secret_str: String,
    puk: EllipticPoint,
    prk: BigInt,
}

impl ECDSA {
    pub fn new(secret: String) -> ECDSA {
        let params = Secp256k1Param::new();
        let prv = hash(secret.as_str(), Some(&(params.n() - &BigInt::from(1))));
        let pk = params.generator() * &prv;
        ECDSA {
            secret_str: secret,
            puk: pk,
            prk: prv.num,
        }
    }
    // serialize the public key using uncompressed sec format using hex
    pub fn sec(&self) -> String {
        let x = self.puk.x.num.to_str_radix(16);
        let y = self.puk.y.num.to_str_radix(16);

        let mut x = x.clone();
        let mut y = y.clone();
        if x.len() < 64 {
            let mut zeros = String::new();
            for _ in 0..(64 - x.len()) {
                zeros.push('0');
            }
            x = zeros + &x;
        }

        if y.len() < 64 {
            let mut zeros = String::new();
            for _ in 0..(64 - y.len()) {
                zeros.push('0');
            }
            y = zeros + &y;
        }

        let mut sec = String::new();
        sec.push_str("04");
        sec.push_str(&x);
        sec.push_str(&y);
        sec
    }

    // serialize the public key using compressed sec format
    pub fn sec_comp(&self) -> String {
        // convert the x coordinate to hex
        let x = self.puk.x.num.to_str_radix(16);
        // get the prefix (even or odd y coordinate..02 or 03)
        let prefix = if &self.puk.y.num % &BigInt::from(2) == BigInt::from(0) {
            "02"
        } else {
            "03"
        };

        // pad the x coordinate with zeros if it is less than 64 characters
        if x.len() < 64 {
            let mut zeros = String::new();
            for _ in 0..(64 - x.len()) {
                zeros.push('0');
            }
            format!("{}{}", prefix, zeros + &x)
        } else {
            format!("{}{}", prefix, x)
        }
    }




    pub fn secret(&self) -> String {
        self.secret_str.clone()
    }

    pub fn pk(&self) -> EllipticPoint {
        self.puk.clone()
    }

    pub fn prk(&self) -> BigInt {
        self.prk.clone()
    }



    // pub fn parse_sec(sec: &str)
    pub fn sign(&self, z: FieldElement) -> Signature {
        // this struct contains all secp256k1 parameters
        let params = Secp256k1Param::new();
        // get a random 1 < k < n - 1
        let k = BigInt::from(get_rndm(&params.n()));
        // k % prime
        let k = FieldElement {
            num: k,
            prime: params.prime(),
        };

        // R = k * G
        let R = params.generator() * &k;
        // get r.x
        let r = R.x.num.clone();
        // k^-1 = k^(n-2) mod n
        let k_inv = k.num.modpow(&(params.n() - BigInt::from(2)), &params.n());
        // s = k^-1 * (z + r * d) mod n
        let mut s = (k_inv * (z.num.clone() + r.clone() * self.prk())) % &params.n();
        // it's possible to modify s without invalidating the signature
        if s > params.n() / BigInt::from(2) {
            s = params.n() - s;
        }
        Signature { r, s }
    }
    pub fn verify(&self, z: FieldElement, signature: Signature) -> bool {
        let params = Secp256k1Param::new();
        // re-calculate the hash
        // s^-1 = s^(n-2) mod n
        let s_inv = signature
            .s
            .modpow(&(params.n() - BigInt::from(2)), &params.n());
        // u = z * s^-1 mod n
        let u = z.num.clone() * s_inv.clone() % &params.n();
        let v = signature.r.clone() * &s_inv % &params.n();
        let r2 = &(params.generator() * &u) + &(self.pk() * &v);

        println!("r2.x.num: {:?}", r2.x.num);
        println!("signature.r.x.num: {:?}", signature.r);
        &r2.x.num == &signature.r
    }
}
