
mod crypto;
use crypto::field::FieldElement;

fn main() {
    let p = FieldElement::new(7, 13);
    let q = FieldElement::new(6, 13);
    println!("p.num = {}", p.num());
    println!("p.prime = {}", p.prime());
    println!("p == q is {}", p.eq(&q));
}

