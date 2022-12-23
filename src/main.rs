mod crypto;
// mod crypto::elliptic_curve;
use crypto::elliptic_curve::Point;
use crypto::field::FieldElement;
use num::BigInt;

fn main() {
    let p = FieldElement::new(BigInt::from(3), BigInt::from(13));
    // let q = FieldElement::new(6, 13);
    let q = FieldElement::new(BigInt::from(6), BigInt::from(13));

    println!("p.num = {}", p.num());
    println!("q.num = {}", q.num());
    println!("p.prime = {}", p.prime());
    println!("p == q is {}", p.eq(&q));
    let res = p.add(&q);
    println!("p + q = {}", res.num());
    // substracing q - p
    let res2 = q.substract(&p);
    println!("q - p = {}", res2.num());
    let p1 = FieldElement::new(BigInt::from(3), BigInt::from(13));
    let p2 = FieldElement::new(BigInt::from(12), BigInt::from(13));
    let res3 = FieldElement::new(BigInt::from(10), BigInt::from(13));

    // let p1 = FieldElement::new(3, 13);
    // let p2 = FieldElement::new(12, 13);
    // let res3 = FieldElement::new(10, 13);

    assert!(p1.multiply(&p2).eq(&res3));
    println!("p1 * p2 = {}", p1.multiply(&p2).num());
    println!("p1 * p2 = {}", res3.num());
    println!("p1.pow(3) = {}", p1.pow(BigInt::from(3)).num());

    // the curve is y**2 = x**3 + 7, secp256k1

    // this point is on the curve
    let x = Point::new(
        FieldElement::new(BigInt::from(15), BigInt::from(31)),
        FieldElement::new(BigInt::from(29), BigInt::from(31)),
        FieldElement::new(BigInt::from(0), BigInt::from(31)),
        FieldElement::new(BigInt::from(7), BigInt::from(31)),
        // FieldElement::new(15, 31),
        // FieldElement::new(29, 31),
        // FieldElement::new(0, 31),
        // FieldElement::new(7, 31),
    );
    // this point is not on the curve
    // let y = Point::new(
    //     FieldElement::new(18, 31),
    //     FieldElement::new(77, 31),
    //     FieldElement::new(0, 31),
    //     FieldElement::new(7, 31),
    // );
    // println!("x == y is {}", x.eq(&y));
}
