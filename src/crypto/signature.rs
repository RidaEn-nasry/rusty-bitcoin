use std::fmt::format;

// implementatation of the ECDSA signature algorithm
// r : x-coordinate of the point R
// s : k^-1 * (z + r * d) mod n
// z : hash of the message
use num::BigInt;

use super::EllipticPoint;
use crate::crypto::field::FieldElement;

// #[derive(Debug, Clone)]
pub struct Signature {
    pub r: BigInt,
    pub s: BigInt,
}

impl Signature {
    pub fn new(r: BigInt, s: BigInt) -> Signature {
        Signature { r, s }
    }

    #[allow(dead_code)]
    // converting vec of raw bytes to hex string
    fn vec_to_hexb(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
    // encoding the signature in DER format
    pub fn der(&self) -> String {
        // get raw bytes of r and s
        let (raw_r, raw_s) = (self.r.to_bytes_be().1, self.s.to_bytes_be().1);
        // get whather r and s high bit is set
        let (high_r, high_s) = (raw_r[0] & 0x80 == 0x80, raw_s[0] & 0x80 == 0x80);
        // get s and r as hex big endian strings
        let (r, s) = (Self::vec_to_hexb(&raw_r), Self::vec_to_hexb(&raw_s));
        // removing the leading zeros
        let (r, s) = (r.trim_start_matches('0'), s.trim_start_matches('0'));
        // if high bit is set, add a zero byte
        let (r, s) = (
            if high_r {
                format!("00{}", r)
            } else {
                r.to_string()
            },
            if high_s {
                format!("00{}", s)
            } else {
                s.to_string()
            },
        );
        // get length of both r and s in hex
        let (r_len, s_len) = (
            format!("{:02x}", r.len() / 2),
            format!("{:02x}", s.len() / 2),
        );
        // Concatenate r, r, and the appropriate marker bytes and length fields
        let result = format!("02{}{}02{}{}", r_len, r, s_len, s);
        // add the sequence marker and length field
        let res_len = format!("{:02x}", result.len() / 2);
        format!("30{}", res_len) + &result
    }
}
