# ecole42-matrix

A basic linear algebra package in Rust, implementing core vector and matrix operations with unit tests.

## Project Overview

This project is implemented following the "Enter the Matrix: An Introduction to Linear Algebra" [document](./assets/en.subject.pdf), with exercises designed to build a foundational understanding of linear algebraic concepts through code. The library covers:

- Vectors and matrices
- Basic scalar operations: addition, subtraction, scaling
- Linear combinations and interpolation
- Dot product, cross product (for 3D vectors)
- Various norms (1-norm, 2-norm, infinity-norm)
- Cosine of angle between vectors
- Matrix multiplication (with vectors and matrices)
- Trace, transpose, determinant
- Row-echelon form, rank, inverse calculations

**Note:** This implementation does **not** include the bonus exercises covering complex vector spaces or projection matrices.

## Features

- Generic struct implementation for scalar operations
- No external mathematical libraries used, no `std` math library for forbidden operations
- Functions adhere to time/space complexities specified in project documentation

## Example Usage

A complete file of minimal usage examples is provided in [EXAMPLES.md](EXAMPLES.md).  
This covers how to create vectors and matrices, use macros like `vector!` and `matrix!`, and apply basic arithmetic and matrix operations.

## Testing

Each implemented function comes with a unit test demonstrating the expected input and output format. Run with:

```bash
cargo test
```