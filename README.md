# qmatrix

qmatrix is a simple matrix implementation for Rust. It includes matrix adition, subtration, multiplication, exponentiation and filling.

## Example

See ```demo.rs``` to know the basic instructions; it is quick to understand and you will get important knowledge if you want to use this in your project or to modify it. Here is an example of how it is structured:

```rust
let mut m1 = QMatrix::new(10, 10);
m1.fill_with_c();

let mut m2 = QMatrix::new(10, 10);
m2.fill_with_d();

let res = m1.mul(&m2);
res.print();
```

## Compile

To compile the demo, run ```cargo run --example demo``` in the working directory.
