use crate::crypto::field::FieldElement;
use num::bigint::BigInt;

#[derive(Clone)]
pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Point({}, {}) on curve y^2 = x^3 + {}x + {}",
            self.x.num(),
            self.y.num(),
            self.a.num(),
            self.b.num()
        )
    }
}

impl Point {
    // a private method to check if a point is on the curve
    #[allow(dead_code)]
    fn on_curve(&self) -> bool {
        //y^2 == x^3 + a*x + b
        self.y.pow(BigInt::from(2)).eq(&self
            .x
            .pow(BigInt::from(3))
            .add(&self.a.multiply(&self.x))
            .add(&self.b))
    }
    // our constructor either returns a point or panics
    pub fn new(x: FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Point {
        let p = Point { x, y, a, b };
        if !p.on_curve() {
            panic!("Point ({}, {}) is not on the curve", p.x.num(), p.y.num());
        }
        p
    }
    // checking equality of two points
    pub fn eq(&self, other: &Point) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y) && self.a.eq(&other.a) && self.b.eq(&other.b)
    }

    // checking if point is at infinity
    pub fn is_infinity(&self) -> bool {
        self.x.num() == BigInt::from(0) && self.y.num() == BigInt::from(0)
    }

    #[allow(dead_code)]
    pub fn infinity(a: FieldElement, b: FieldElement) -> Point {
        Point::new(
            FieldElement::new(BigInt::from(0), a.prime()),
            FieldElement::new(BigInt::from(0), a.prime()),
            a,
            b,
        )
    }

    pub fn add(&self, other: &Point) -> Self {
        // if the points are not on the same curve, we can't add them
        if self.x.num() != other.x.num() || self.y.num() != other.y.num() {
            panic!(
                "{}, {} cannot be added as they're not on the same curve",
                self, other
            )
        }
        // if both points are in infinity, we return infinity (0, 0)
        if self.is_infinity() && other.is_infinity() {
            return Self::infinity(self.a.clone(), self.b.clone());
        }
        if self.is_infinity() {
            // (0, 0) + (x, y) = (x, y)
            return other.clone();
        } else if other.is_infinity() {
            // (x, y) + (0, 0) = (x, y)
            return self.clone();
        }
        // if the points are additive inverses (p + (-p) = 0) when x1 == x2 and y1 != y2
        if self.x.eq(&other.x) && self.y.substract(&other.y).num() == BigInt::from(0) {
            return Self::infinity(self.a.clone(), self.b.clone());
        }
        // when x1 != x2
        else if self.x.num() != other.x.num() {
            // slope = (y2 - y1) / (x2 - x1)
            let s = other
                .y
                .substract(&self.y)
                .divide(&other.x.substract(&self.x));
            // x3 = s^2 - x1 - x2
            let x3 = s
                .pow(BigInt::from(2))
                .substract(&self.x)
                .substract(&other.x);
            // y3 = s * (x1 - x3) - y1
            let y3 = s.multiply(&self.x.substract(&x3)).substract(&self.y);
            // return the new point
            return Point::new(x3, y3, self.a.clone(), self.b.clone());
        }
        // if points are the same we use tangent
        if self.eq(other) {
            // if p1 == p2 and p1.y == 0, then p1 + p2 = 0
            if self.y.num() == BigInt::from(0) {
                return Self::infinity(self.a.clone(), self.b.clone());
            } else {
                // slope = (3 * x1^2 + a) / (2 * y1)
                let s = self
                    .x
                    .pow(BigInt::from(2))
                    .multiply(&FieldElement::new(BigInt::from(3), self.x.prime()))
                    .add(&self.a)
                    .divide(
                        &self
                            .y
                            .multiply(&FieldElement::new(BigInt::from(2), self.y.prime())),
                    );
                // x3 = s^2 - 2 * x1
                let x3 = s.pow(BigInt::from(2)).substract(
                    &self
                        .x
                        .multiply(&FieldElement::new(BigInt::from(2), self.x.prime())),
                );
                // y3 = s * (x1 - x3) - y1
                let y3 = s.multiply(&self.x.substract(&x3)).substract(&self.y);
                // return the new point
                return Point::new(x3, y3, self.a.clone(), self.b.clone());
            }
        } else {
            panic!(
                "Something went wrong while adding points {} and {}",
                self, other
            );
        }
    }
    // scalar multiplication
    pub fn multiply(&self, num: &FieldElement) -> Self {
        // better calculate the thing using binary expantion, a reduction from O(n) complexity to O(log n)
        let mut result = Self::infinity(self.a.clone(), self.b.clone());
        let mut current = self.clone();
        let mut exp = num.clone();
        while exp.num() > BigInt::from(0) {
            // if most significant bit (the leftmost bit) is 1 , then add the current point to the result
            if exp.num().bits() & BigInt::from(1).bits() == BigInt::from(1).bits() {
                result = result.add(&current);
            }
            // double the current point
            current = current.add(&current);
            // shift exp to the right by 1 bit
            exp = exp.divide(&FieldElement::new(BigInt::from(2), exp.prime()));
        }
        result
    }
}

