// impl Add for FieldElement {
//     type Output = FieldElement;
//     fn add(self, other: FieldElement) -> FieldElement {
//         if self.prime != other.prime {
//             panic!("Cannot add two numbers in different Fields");
//         }
//         FieldElement {
//             num: (&self.num + &other.num) % &self.prime,
//             prime: (self.prime),
//         }
//     }
// }

// impl Sub for FieldElement {
//     type Output = FieldElement;
//     fn sub(self, other: FieldElement) -> FieldElement {
//         if self.prime != other.prime {
//             panic!("Cannot substract two numbers in different Fields");
//         }
//         FieldElement {
//             num: (&self.num + &self.prime - &other.num) % &self.prime,
//             prime: (self.prime),
//         }
//     }
// }
// impl Div for FieldElement {
//     type Output = FieldElement;
//     fn div(self, other: FieldElement) -> FieldElement {
//         if self.prime != other.prime {
//             panic!("Cannot divide two numbers in different Fields");
//         }
//         // if other is zero, panic
//         if other.num == BigInt::from(0) {
//             panic!("Cannot divide by zero");
//         }
//         // using the fact that a/b = a * b^-1 or a * b^(p-2) (fermat's little theorem)
//         let num = (&self.num * &other.num.modpow(&(&self.prime - 2), &self.prime)) % &self.prime;
//         FieldElement {
//             num,
//             prime: self.prime,
//         }
//     }
// }

// impl PartialEq for FieldElement {
//     fn eq(&self, other: &FieldElement) -> bool {
//         if self.prime != other.prime {
//             panic!("Cannot compare two numbers in different Fields");
//         }
//         self.num == other.num
//     }
// }

// impl Mul for FieldElement {
//     type Output = FieldElement;
//     fn mul(self, other: FieldElement) -> FieldElement {
//         if self.prime != other.prime {
//             panic!("Cannot multiply two numbers in different Fields");
//         }
//         FieldElement {
//             num: (&self.num * &other.num) % &self.prime,
//             prime: (self.prime),
//         }
//     }
// }

// a version of mul for FieldElement and BigInt
// impl Mul<BigInt> for FieldElement {
//     type Output = FieldElement;
//     fn mul(self, other: BigInt) -> FieldElement {
//         let other = FieldElement {
//             num: other,
//             prime: self.prime.clone(),
//         };
//         FieldElement {
//             num: (&self.num * &other.num) % &self.prime,
//             prime: (self.prime),
//         }
//     }
// }

// a version of mul for i32 and FieldElement
// impl Mul<i32> for FieldElement {
//     type Output = FieldElement;
//     fn mul(self, other: i32) -> FieldElement {
//         let other = FieldElement {
//             num: BigInt::from(other),
//             prime: self.prime.clone(),
//         };
//         FieldElement {
//             num: (&self.num * &other.num) % &self.prime,
//             prime: (self.prime),
//         }
//     }
// }

use num::bigint::BigInt;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl<'a> Add<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        FieldElement {
            num: (&self.num + &other.num) % &self.prime,
            prime: (self.prime),
        }
    }
}

impl<'a> Sub<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn sub(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot substract two numbers in different Fields");
        }
        FieldElement {
            num: (&self.num + &self.prime - &other.num) % &self.prime,
            prime: (self.prime),
        }
    }
}

impl<'a> PartialEq<&'a FieldElement> for FieldElement {
    fn eq(&self, other: &&'a FieldElement) -> bool {
        if self.prime != other.prime {
            panic!("Cannot compare two numbers in different Fields");
        }
        self.num == other.num
    }
}

impl<'a> Div<&'a FieldElement> for FieldElement {
    type Output = FieldElement;

    fn div(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        // if other is zero, panic
        if other.num == BigInt::from(0) {
            panic!("Cannot divide by zero");
        }
        // using the fact that a/b = a * b^-1 or a * b^(p-2) (fermat's little theorem)
        let num = (&self.num * &other.num.modpow(&(&self.prime - 2), &self.prime)) % &self.prime;
        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

impl<'a> Mul<&'a FieldElement> for FieldElement {
    type Output = FieldElement;
    fn mul(self, other: &'a FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        FieldElement {
            num: (&self.num * &other.num) % &self.prime,
            prime: (self.prime),
        }
    }
}

//a version of mul for FieldElement and BigInt
impl<'a> Mul<&'a BigInt> for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: &'a BigInt) -> FieldElement {
        let other = FieldElement {
            num: other.clone(),
            prime: self.prime.clone(),
        };
        FieldElement {
            num: (&self.num * &other.num) % &self.prime,
            prime: (self.prime),
        }
    }
}

// a version of mul for i32 and FieldElement
impl<'a> Mul<&'a i32> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &'a i32) -> Self::Output {
        let other = FieldElement {
            num: BigInt::from(*rhs),
            prime: self.prime.clone(),
        };
        FieldElement {
            num: (&self.num * &other.num) % &self.prime,
            prime: (self.prime),
        }
    }
}

impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> FieldElement {
        // if num >= prime || prime <= BigInt::from(0) {
        //     panic!("Num {} not in field range 0 to {}", num, prime - 1);
        // }
        let num = num % &prime;
        FieldElement { num, prime }
    }
    // pub fn num(&self) -> BigInt {
    //     self.num.clone()
    // }
    // pub fn prime(&self) -> BigInt {
    //     self.prime.clone()
    // }
    pub fn pow(&self, exponent: &BigInt) -> FieldElement {
        let mut n = exponent.clone();
        while n < BigInt::from(0) {
            n = n + &self.prime - 1;
        }
        let num = self.num.modpow(&n, &self.prime);
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }
    // an overload of pow for i32
    pub fn pow_i32(&self, exponent: i32) -> FieldElement {
        let mut n = BigInt::from(exponent);
        while n < BigInt::from(0) {
            n = n + &self.prime - 1;
        }
        let num = self.num.modpow(&n, &self.prime);
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }

    // sqrt

    pub fn sqrt(&self) -> Option<FieldElement> {
        if &self.prime % 4 != BigInt::from(3) {
            panic!("Cannot find square root of {} in this field", self.num);
        }
        let num = self.pow(&((&self.prime + 1) / 4));
        Some(FieldElement {
            num: num.num,
            prime: self.prime.clone(),
        })
    }
}
