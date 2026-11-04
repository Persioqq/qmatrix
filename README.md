# qmatrix

qmatrix is a simple matrix implementation for Rust. It includes matrix adition, subtration, multiplication, exponentiation and filling.

## Example

See ```demo.rs``` to know the basic instructions; it is quick to understand and you will get important knowledge if you want to use this in your project or to modify it. Here is an example of how it is structured:

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

## Compile

To compile the demo, run ```cargo run --example demo``` in the working directory.
