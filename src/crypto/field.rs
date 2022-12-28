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

    pub fn divide(&self, other: &FieldElement) -> FieldElement {
        // if elements are not in the same field, panic
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different fields");
        }
        // if other is zero, panic
        if other.num == BigInt::from(0) {
            panic!("Cannot divide by zero");
        }
        // using the fact that a/b = a * b^-1 or a * b^(p-2) (fermat's little theorem)
        let num = (&self.num * &other.num.modpow(&(&self.prime - 2), &self.prime)) % &self.prime;
        FieldElement {
            num,
            prime: self.prime.clone(),
        }
    }
}
