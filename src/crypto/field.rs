pub struct FieldElement {
    num: u128,
    prime: u128,
}

impl FieldElement {
    pub fn new(num: u128, prime: u128) -> FieldElement {
        if num >= prime || prime <= 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }
    pub fn num(&self) -> u128 {
        self.num
    }
    pub fn prime(&self) -> u128 {
        self.prime
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
            num: (self.num + other.num) % self.prime,
            prime: (self.prime),
        }
    }
    pub fn substract(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot substract two numbers in different Fields");
        }
        FieldElement {
            num: (self.num + self.prime - other.num) % self.prime,
            prime: (self.prime),
        }
    }
    pub fn multiply(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        FieldElement {
            num: (self.num * other.num) % self.prime,
            prime: (self.prime),
        }
    }
    pub fn pow(&self, exponent: u128) -> FieldElement {
        let mut result = FieldElement::new(1, self.prime);
        let mut base = self.clone();
        let mut exp = exponent;
        //
        for _ in 0..exp {
            result = result.multiply(&base)
        }
        result
    }
    pub fn divide(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }
        // use Fermat's little theorem
        // self.num**(p-1) % p == 1
        let mut other_inv = other.pow(self.prime - 2);
        self.multiply(&other_inv)
    }


}
