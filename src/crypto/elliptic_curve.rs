// use core::num::fmt::Part;
use std::ops::{Shr, Sub};

use crate::crypto::field::FieldElement;
use num::bigint::{BigInt, Sign};
use std::cmp::PartialEq;
use std::ops::{Add, Mul};

// overloading Operators for EllipticPoint
// impl PartialEq for EllipticPoint {
//     fn eq(&self, other: &EllipticPoint) -> bool {
//         self.x == &other.x && self.y ==&other.y && self.a == &other.a && self.b == &other.b
//     }
// }

#[derive(Clone)]
pub struct EllipticPoint {
    pub x: FieldElement,
    pub y: FieldElement,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl std::fmt::Display for EllipticPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "EllipticPoint({}, {}) on curve y^2 = x^3 + {}x + {}",
            &self.x.num, &self.y.num, &self.a.num, &self.b.num
        )
    }
}

impl<'a, 'b> PartialEq<&'b EllipticPoint> for &'a EllipticPoint {
    fn eq(&self, other: &&'b EllipticPoint) -> bool {
        self.x == &other.x && self.y == &other.y && self.a == &other.a && self.b == &other.b
    }
}

impl<'a, 'b> Add<&'b EllipticPoint> for &'a EllipticPoint {
    type Output = EllipticPoint;
    fn add(self, other: &'b EllipticPoint) -> EllipticPoint {
        if self.a != &other.a || self.b != &other.b {
            panic!("Points {}, {} are not on the same curve", self, other);
        }
        if self.is_infinity() {
            return other.clone();
        }
        if other.is_infinity() {
            return self.clone();
        }
        // if x1 == x2 and y1 != y2
        if self.x.eq(&&other.x)
            && self.y.eq(&&FieldElement::new(
                &other.y.num * BigInt::from(-1),
                self.a.prime.clone(),
            ))
        {
            return EllipticPoint::infinity(self.a.clone(), self.b.clone());
        } else {
            let slope = if self.x == &other.x {
                // slope = (3x1^2 + a) / (2y1)
                (self.x.pow_i32(2) * &3 + &self.a) / &(self.y.clone() * &2i32)
                // self.x
                //     .pow(&BigInt::from(2))
                //     .multiply(&FieldElement::new(BigInt::from(3), self.a.prime()))
                //     .add(&self.a)
                //     .divide(
                //         &self
                //             .y
                //             .multiply(&FieldElement::new(BigInt::from(2), self.a.prime())),
                //     )
            } else {
                // slope = (y2 - y1) / (x2 - x1)
                (other.y.clone() - &self.y) / &(other.x.clone() - &self.x)
            };
            let x3 = slope.pow_i32(2) - &self.x - &other.x;
            let y3 = slope * &(self.x.clone() - &x3) - &self.y;
            EllipticPoint {
                x: x3,
                y: y3,
                a: self.a.clone(),
                b: self.b.clone(),
            }
        }
    }
}
// impl<'a, 'b> Add<&'b EllipticPoint> for &'a EllipticPoint {
//     type Output = EllipticPoint;
//     fn add(self, other: &'b EllipticPoint) -> EllipticPoint {
//         if self.a != other.a || self.b != other.b {
//             panic!("Points {}, {} are not on the same curve", self, other);
//         }
//         if self.is_infinity() {
//             return *other;
//         }
//         if other.is_infinity() {
//             return self;
//         }
//         // if x1 == x2 and y1 != y2
//         if self.x.eq(&other.x)
//             && self.y.eq(&FieldElement::new(
//                 &other.y.num() * BigInt::from(-1),
//                 self.a.prime(),
//             ))
//         {
//             return &EllipticPoint::infinity(self.a.clone(), self.b.clone());
//         } else {
//             let slope = if self.x == other.x {
//                 // slope = (3x1^2 + a) / (2y1)
//                 self.x.pow_i32(2) * 3 + self.a / (self.y * 2)
//                 // self.x
//                 //     .pow(&BigInt::from(2))
//                 //     .multiply(&FieldElement::new(BigInt::from(3), self.a.prime()))
//                 //     .add(&self.a)
//                 //     .divide(
//                 //         &self
//                 //             .y
//                 //             .multiply(&FieldElement::new(BigInt::from(2), self.a.prime())),
//                 //     )
//             } else {
//                 // slope = (y2 - y1) / (x2 - x1)
//                 (other.y - self.y) / (other.x - self.x)
//                 // other
//                 //     .y
//                 //     .substract(&self.y)
//                 //     .divide(&other.x.substract(&self.x))
//             };
//             let x3 = slope.pow_i32(2) - self.x.clone() - other.x;

//             // let x3 = slope
//             //     .pow(&BigInt::from(2))
//             //     .substract(&self.x)
//             //     .substract(&other.x);
//             let y3 = slope * (self.x.clone() - x3) - self.y.clone();
//             // let y3 = slope.multiply(&self.x.substract(&x3)).substract(&self.y);
//             return &EllipticPoint {
//                 x: x3,
//                 y: y3,
//                 a: self.a.clone(),
//                 b: self.b.clone(),
//             };
//         }
//     }
// }
// impl Add for EllipticPoint {
//     type Output = Self;
//     fn add(self, other: &Self) -> Self {
//         if self.a != other.a || self.b != other.b {
//             panic!("Points {}, {} are not on the same curve", self, other);
//         }
//         if self.is_infinity() {
//             return *other;
//         }
//         if other.is_infinity() {
//             return self;
//         }
//         // if x1 == x2 and y1 != y2
//         if self.x.eq(&other.x)
//             && self.y.eq(&FieldElement::new(
//                 &other.y.num() * BigInt::from(-1),
//                 self.a.prime(),
//             ))
//         {
//             return Self::infinity(self.a.clone(), self.b.clone());
//         } else {
//             let slope = if self.x == other.x {
//                 // slope = (3x1^2 + a) / (2y1)
//                 self.x.pow_i32(2) * 3 + self.a / (self.y * 2)
//                 // self.x
//                 //     .pow(&BigInt::from(2))
//                 //     .multiply(&FieldElement::new(BigInt::from(3), self.a.prime()))
//                 //     .add(&self.a)
//                 //     .divide(
//                 //         &self
//                 //             .y
//                 //             .multiply(&FieldElement::new(BigInt::from(2), self.a.prime())),
//                 //     )
//             } else {
//                 // slope = (y2 - y1) / (x2 - x1)
//                 (other.y - self.y) / (other.x - self.x)
//                 // other
//                 //     .y
//                 //     .substract(&self.y)
//                 //     .divide(&other.x.substract(&self.x))
//             };
//             let x3 = slope.pow_i32(2) - self.x.clone() - other.x;

//             // let x3 = slope
//             //     .pow(&BigInt::from(2))
//             //     .substract(&self.x)
//             //     .substract(&other.x);
//             let y3 = slope * (self.x.clone() - x3) - self.y.clone();
//             // let y3 = slope.multiply(&self.x.substract(&x3)).substract(&self.y);
//             return Self::new(x3, y3, self.a.clone(), self.b.clone());
//         }
//     }
// }

// impl Mul<FieldElement> for EllipticPoint {
//     type Output = Self;

//     fn mul(self, rhs: FieldElement) -> Self::Output {
//         let mut result = Self::infinity(self.a.clone(), self.b.clone());
//         let mut current = self.clone();
//         let mut exp = rhs.clone();
//         while exp.num() != BigInt::from(0) {
//             // using double-add algorithm
//             // if most significant bit (the leftmost bit) is 1 , then add the current EllipticPoint to the result
//             if exp.num.bit(0) == true {
//                 result = result + current.clone();
//             }
//             // double the currentEllipticPoint
//             current = current.clone() + current.clone();
//             // shift exp to the right by 1 bit
//             exp.num = exp.num.shr(1);
//         }
//         result
//     }
// }

impl<'a> Mul<&'a FieldElement> for EllipticPoint {
    type Output = Self;

    fn mul(self, rhs: &'a FieldElement) -> Self::Output {
        let mut result = Self::infinity(self.a.clone(), self.b.clone());
        let mut current = self.clone();
        let mut exp = rhs.clone();
        while exp.num != BigInt::from(0) {
            // using double-add algorithm
            // if most significant bit (the leftmost bit) is 1 , then add the current EllipticPoint to the result
            if exp.num.bit(0) == true {
                result = &result + &current.clone();
            }
            // double the currentEllipticPoint
            current = &current.clone() + &current.clone();
            // shift exp to the right by 1 bit
            exp.num = exp.num.shr(1);
        }
        result
    }
}

// a version of Mul that takes a BigInt as rhs
// impl Mul<BigInt> for EllipticPoint {
//     type Output = Self;
//     fn mul(self, rhs: BigInt) -> Self::Output {
//         let rhs = FieldElement::new(rhs, self.a.prime());
//         self * rhs
//     }
// }

impl<'a> Mul<&'a BigInt> for EllipticPoint {
    type Output = Self;
    fn mul(self, rhs: &'a BigInt) -> Self::Output {
        let rhs = FieldElement::new(rhs.clone(), self.a.prime.clone());
        self * &rhs
    }
}

impl EllipticPoint {
    // a private method to check if a EllipticPoint is on the curve
    #[allow(dead_code)]
    fn on_curve(&self) -> bool {
        //y^2 == x^3 + a*x + b
        self.y.pow(&BigInt::from(2))
            == &(self.x.pow(&BigInt::from(3))
                + &(self.a.clone() * &self.x.clone())
                + &self.b.clone())
        // self.y.pow(&BigInt::from(2)).eq(&self
        //     .x
        //     .pow(&BigInt::from(3))
        //     .add(&self.a.multiply(&self.x))
        //     .add(&self.b))
    }
    // our constructor either returns a EllipticPoint or panics
    pub fn new(
        x: FieldElement,
        y: FieldElement,
        a: FieldElement,
        b: FieldElement,
    ) -> EllipticPoint {
        let p = EllipticPoint { x, y, a, b };
        if p.is_infinity() {
            return p;
        }

        if !p.on_curve() {
            panic!(
                "EllipticPoint ({}, {}) is not on the curve",
                &p.x.num, &p.y.num
            );
        }
        p
    }
    /// getters
    pub fn x(&self) -> FieldElement {
        self.x.clone()
    }
    pub fn y(&self) -> FieldElement {
        self.y.clone()
    }

    // checking equality of two EllipticPoints
    // pub fn eq(&self, other: &EllipticPoint) -> bool {
    //     self.x.eq(&other.x) && self.y.eq(&other.y) && self.a.eq(&other.a) && self.b.eq(&other.b)
    // }

    // checking if EllipticPoint is at infinity
    pub fn is_infinity(&self) -> bool {
        &self.x.num == &BigInt::from(0) && &self.y.num == &BigInt::from(0)
    }

    #[allow(dead_code)]
    pub fn infinity(a: FieldElement, b: FieldElement) -> EllipticPoint {
        EllipticPoint::new(
            FieldElement::new(BigInt::from(0), a.prime.clone()),
            FieldElement::new(BigInt::from(0), a.prime.clone()),
            a,
            b,
        )
    }

    // pub fn add(&self, other: &EllipticPoint) -> Self {
    //     // implement the point addition algorithm

    //     // P = Self, Q = other
    //     // if P == 0. P + Q = Q
    //     if self.is_infinity() {
    //         return other.clone();
    //     }
    //     // if Q == 0. P + Q = P
    //     if other.is_infinity() {
    //         return self.clone();
    //     }

    //     }
    // }

    // scalar multiplication
    // pub fn multiply(&self, num: &FieldElement) -> Self {
    //     // calculating the scalar multiplication using regular looping
    //     // let mut result = Self::infinity(self.a.clone(), self.b.clone());
    //     // // let mut current = self.clone();
    //     // let mut exp = num.clone();
    //     // while exp.num() > BigInt::from(0) {
    //     //     result = result.add(&self);
    //     //     exp.num = exp.num - &BigInt::from(1);
    //     // }
    //     // result

    //     // // better calculate the thing using binary expantion, a reduction from O(n) complexity to O(log n)
    //     let mut result = Self::infinity(self.a.clone(), self.b.clone());
    //     let mut current = self.clone();
    //     let mut exp = num.clone();
    //     while exp.num() != BigInt::from(0) {
    //         // using double-add algorithm
    //         // if most significant bit (the leftmost bit) is 1 , then add the current EllipticPoint to the result
    //         if exp.num.bit(0) == true {
    //             result = result.add(&current);
    //         }
    //         // double the currentEllipticPoint
    //         current = current.add(&current);
    //         // shift exp to the right by 1 bit
    //         exp.num = exp.num.shr(1);
    //     }
    //     result
    // }
}
