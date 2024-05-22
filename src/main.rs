use matrix::matrix::matrix;
use matrix::matrix::Matrix; // Needed to be able to use matrix! macro
use matrix::vector::Vector;
use matrix::VectorSpace; // Needed to be able to use trait methods

fn main() {
    let mut v = Vector::from(vec![1., 2.5, 2.]);
    let b = Vector::from(vec![2., -2., 4.]);
    println!("{v}");

    for i in &v {
        println!("Loop for iter {i}");
    }

    v.add(&b);
    println!("v: {v} b: {b}");

    v += &b;
    println!("v: {v} b: {b}");

    let m = matrix![[1, 2, 3], [7, 8, 9], [10, 110, 12], [20, 25, 26]];
    println!("\nmatrix display:\n{m}");
}
