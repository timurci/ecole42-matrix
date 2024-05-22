use matrix::matrix::matrix;
use matrix::vector::vector;
use matrix::VectorSpace; // Needed to be able to use trait methods

fn main() {
    let mut v = vector![1., 2.5, 2.];
    let b = vector![2., -2., 4.];
    println!("{v}");

    for i in &v {
        println!("Loop for iter {i}");
    }

    v.add(&b);
    println!("v: {v} b: {b}");

    v += &b;
    println!("v: {v} b: {b}");

    let mut m = matrix![
        [1., 2., 3.],
        [7., 8., 9.],
        [10., 110., 12.],
        [20.65, 25.11, 26.2]
    ];
    let c = matrix![5.; 4,3];
    println!("\nmatrix display:\n{m}");

    m.add(&c);
    println!("m:\n{m}\nc:\n{c}");

    m.scl(-0.5);
    println!("m:\n{m}\nscl: -0.5");
}
