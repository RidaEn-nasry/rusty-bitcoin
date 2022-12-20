pub struct FieldElement {
    num: u8,

    prime: u8,
}
impl FieldElement {
    pub fn new(num: u8, prime: u8) -> FieldElement {
        if num >= prime || prime <= 0 {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        FieldElement { num, prime }
    }

    pub fn num(&self) -> u8 {
        self.num
    }

    pub fn prime(&self) -> u8 {
        self.prime
    }

    pub fn eq(&self, other: &FieldElement) -> bool {
        if self.prime != other.prime {
            panic!("Cannot compare two numbers in different Fields");
        }
        self.num == other.num
    }
}
