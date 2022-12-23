use num::bigint::BigInt;

#[derive(Clone)]
pub struct FieldElement {
    pub num: BigInt,
    pub prime: BigInt,
}


impl FieldElement {
    pub fn new(num: BigInt, prime: BigInt) -> FieldElement {
        if num >= prime || prime <= BigInt::from(0) {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }
    pub fn num(&self) -> BigInt {
        self.num.clone()
    }
    pub fn prime(&self) -> BigInt {
        self.prime.clone()
    }
    pub fn eq(&self, other: &FieldElement) -> bool {
        if self.prime != other.prime {
            panic!("Cannot compare two numbers in different Fields");
        }
        self.num == other.num
    }
    pub fn add(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        FieldElement {
            num: (self.num() + other.num()) % self.prime(),
            prime: (self.prime()),
        }
    }
    pub fn substract(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot substract two numbers in different Fields");
        }
        FieldElement {
            num: (self.num() + self.prime() - other.num()) % self.prime(),
            prime: (self.prime()),
        }
    }
    pub fn multiply(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        FieldElement {
            num: (self.num() * other.num()) % self.prime(),
            prime: (self.prime()),
        }
    }
    pub fn pow(&self, exponent: BigInt) -> FieldElement {
        let mut result = FieldElement::new(BigInt::from(1), self.prime());
        let mut base = self.clone();
        let mut exp = exponent;
        //
        for _ in 0..exp.bits() {
            result = result.multiply(&base)
        }
        result
    }
    pub fn divide(&self, other: &FieldElement) -> FieldElement {
        if self.prime() != other.prime() {
            panic!("Cannot divide two numbers in different Fields");
        }
        // use Fermat's little theorem
        // self.num**(p-1) % p == 1
        let mut other_inv = other.pow(self.prime() - BigInt::from(2));
        self.multiply(&other_inv)
    }
}


