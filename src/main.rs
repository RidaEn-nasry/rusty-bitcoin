mod crypto;
use crypto::field::FieldElement;

fn main() {
    let p = FieldElement::new(7, 13);
    let q = FieldElement::new(6, 13);
    println!("p.num = {}", p.num());
    println!("q.num = {}", q.num());
    println!("p.prime = {}", p.prime());
    println!("p == q is {}", p.eq(&q));
    let res = p.add(&q);
    println!("p + q = {}", res.num());
    // substracing q - p
    let res2 = q.substract(&p);
    println!("q - p = {}", res2.num());

    let p1 = FieldElement::new(3, 13);
    let p2 = FieldElement::new(12, 13);
    let res3 = FieldElement::new(10, 13);

    assert!(p1.multiply(&p2).eq(&res3));
    println!("p1 * p2 = {}", p1.multiply(&p2).num());
    println!("p1 * p2 = {}", res3.num());
    println!("p1.pow(3) = {}", p1.pow(3).num());
}
