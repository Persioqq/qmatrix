# qmatrix

qmatrix is a simple matrix implementation for Rust and C. It includes matrix adition, subtration, multiplication, exponentiation and filling.

## Example

See ```demo.rs``` and ```c/main.c``` for more information. It is quick to understand and you will get important knowledge if you want to use this in your project or to modify it. Here is an example of how it is structured:

In Rust:
```rust
let mut m1 = new_m(10, 10);
fill_m_with_c(&mut m1);
print_m(&m1);

let mut m2 = new_m(10, 10);
fill_m_with_d(&mut m2);
print_m(&m2);

let res = mul_m(&m1, &m2);
print_m(&res);
```

In C:
```c
qmatrix *matrix = new_m(10, 10); // This creates a 10x10 matrix
fill_m_with_c(matrix); // This fills the matrix with crescent numbers
matrix->m[0][0] = 33 // Changes the first element to 33
matrix = mul_m(matrix, matrix); // Multiplies itself
print_m(matrix); // Prints the matrix
```

## Compile

To compile the tests, for rust, run ```cargo run --example demo``` in the working directory; and, for C, run ```make``` in the ```c``` directory.
