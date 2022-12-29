use super::{FieldElement, Secp256k1Param, Signature};
use num::bigint::BigInt;
use num::bigint::Sign;

use rand::{thread_rng, Rng, RngCore};
// use num_bigint::bigint::BigInt;
// use num_bigint::ToBigInt;


pub fn get_rndm(n: &BigInt) -> BigInt {
    // generate a random number between 1 and n - 1
    let mut rng = thread_rng();
    let max = n.clone() - BigInt::from(1);
    let min = BigInt::from(1);
    let bytes = max.to_bytes_be();
    let mut num = BigInt::from_bytes_be(Sign::Plus, &rng.gen::<[u8; 32]>());
    while num > max {
        num = BigInt::from_bytes_be(Sign::Plus, &rng.gen::<[u8; 32]>());
    }
    num

    // let num: BigIn = rng.gen_range(min, max);t
}

// pub fn verify(msg_hash: FieldElement, sig: Signature) -> bool {
//     let params = Secp256k1Param::new();

//     // s^(n-2) mod n
//     // get the inverse of s
//     let s_inv = sig.s.num.modpow(
//         &(msg_hash.prime.clone() - BigInt::from(2)),
//         &msg_hash.prime.clone(),
//     );

//     // u = z * s_inv mod n
//     let u = msg_hash.num.modpow(&s_inv, &msg_hash.prime.clone());
//     // v = r * s_inv mod n
//     let v = sig.r.num.modpow(&s_inv, &msg_hash.prime.clone());

//     // total = u * G + v * P
//     let total = params
//         .generator()
//         .multiply(&FieldElement::new(u, msg_hash.prime.clone()))
//         .add(
//             &params
//                 .generator()
//                 .multiply(&FieldElement::new(v, msg_hash.prime.clone())),
//         );
//     total.x.num == sig.r.num
// }

// pub fn sign(msg_hash: FieldElement, prv_key: BigInt) -> Signature {
//     let params = Secp256k1Param::new();

//     let k = FieldElement {
//         num: BigInt::from(get_rndm()),
//         prime: params.prime().clone(),
//     };

//     // r = k * G
//     let r = params.generator().multiply(&k);
//     // k_inv = k^-1 = k^(n-2) mod n
//     let k_inv = k
//         .num
//         .modpow(&(params.prime().clone() - BigInt::from(2)), &params.prime());

//     // s = k^-1 * (z + r * d) mod n
//     let s = (k_inv * (msg_hash.num() + &r.x.num * prv_key.clone())) % &params.prime();

//     let sign = if s > params.n().clone() / BigInt::from(2) {
//         Signature::new(
//             r.x.clone(),
//             FieldElement {
//                 num: params.n().clone() - s,
//                 prime: params.prime().clone(),
//             },
//         )
//     } else {
//         Signature::new(
//             r.x.clone(),
//             FieldElement {
//                 num: s,
//                 prime: params.prime().clone(),
//             },
//         )
//     };
//     sign
// }
