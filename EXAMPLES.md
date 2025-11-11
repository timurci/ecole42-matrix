# ecole42-matrix Usage Examples

Here are minimal Rust code snippets showing how to use the vector and matrix macros and some core functions, based on the unit tests in `src/vector.rs` and `src/matrix.rs`.

---

## Creating Vectors and Matrices

```rust
// Import the macros from your crate:
use matrix::vector;
use matrix::matrix;

// Define a vector:
let v1 = vector![1, 5, -2, 3];           // [1, 5, -2, 3]
let v2 = vector![924; 5];                // [924, 924, 924, 924, 924]

// Define a matrix:
let m1 = matrix![[1, 2, 3], [4, 5, 6]];  // 2x3 matrix
let m2 = matrix![1, 2, 3, 4];            // interpreted as 2x2 matrix: [[1,3], [2,4]]
let m3 = matrix![-1; 2];                 // 2x2 matrix of -1s
let m4 = matrix![0; 3, 2];               // 3x2 matrix of zeros
```

---

## Vector Arithmetic

```rust
// Addition
let mut v = vector![1, 2, 3];
let o = vector![5, 6, 7];
v.add(&o);       // v: [6, 8, 10]

// Subtraction
let mut v = vector![10, 20, -5];
let o = vector![1, 2, 3];
v.sub(&o);       // v: [9, 18, -8]

// Scalar multiplication
let mut v = vector![1, 2, 3];
v.scl(3);        // v: [3, 6, 9]
```

---

## Matrix Arithmetic

```rust
// Addition
let mut m1 = matrix![1, 2, 3, 4];
let m2 = matrix![4, 3, 2, 1];
m1.add(&m2);     // each element is added

// Scalar multiplication
let mut m = matrix![[1, 2], [3, 4]];
m.scl(2);       // [[2, 4], [6, 8]]

// Matrix multiplication
let a = matrix![[3, -5], [6, 8]];
let b = matrix![[2, 1], [4, 2]];
let prod = a.mul_mat(&b); // prod: [[-14, -7], [44, 22]]

// Vector multiplication (matrix * vector)
let m = matrix![[2, -2], [-2, 2]];
let v = vector![4, 2];
let result = m.mul_vec(&v); // result: [4, -4]
```

---

## Useful Functions

```rust
let v = vector![1., 2., 3.];
let w = vector![4., 5., 6.];

// Dot product
let dp = v.dot(&w); // 32.0

// Norms
let norm_1 = v.norm_1();   // 6.0
let norm_2 = v.norm();     // approx 3.742
let norm_inf = v.norm_inf(); // 3.0

// Angle cosine
let c = angle_cos(&v, &w); // ~0.9746

// Cross product (3D vectors)
let u = vector![1, 2, 3];
let v = vector![4, 5, 6];
let cross = cross_product(&u, &v); // [-3, 6, -3]
```

---

## Matrix-Specific Functions

```rust
let m = matrix![[2, -5, 0], [4, 3, 7], [-2, 3, 4]];

// Trace (sum of diagonal)
let tr = m.trace(); // 9

// Determinant
let d = m.determinant();

// Transpose
let mt = m.transpose();

// Row Echelon Form
let refl = m.row_echelon();

// Inverse (for square matrix)
let inv = m.inverse().unwrap();

// Rank
let r = m.rank();
```

---

## Linear Combination and Interpolation

```rust
// Linear combination
let v1 = vector![1, 2, 3];
let v2 = vector![0, 10, -100];
// 10*v1 - 2*v2
let comb = linear_combination([v1, v2].as_slice(), [10, -2].as_slice()); // [10, 0, 230]

// Linear interpolation (lerp)
let interp = lerp(&vector![2., 1.], &vector![4., 2.], 0.3); // [2.6, 1.3]
```

---

See the crate unit tests for more examples and details.