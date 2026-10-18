# qmatrix

qmatrix is a simple matrix implementation for C. It includes matrix adition, subtration, multiplication, exponentiation and filling.

## Example

See ```main.c``` and read ```qmatrix.h``` for more information. It is quick to understand and you will get important knowledge if you want to use this in your project or to modify it. Here is an example of how it is structured:
```c
qmatrix *matrix = new_m(10, 10); // This creates a 10x10 matrix
fill_m_with_c(matrix); // This fills the matrix with crescent numbers
matrix->m[0][0] = 33 // Changes the first element to 33
matrix = mul_m(matrix, matrix); // Multiplies itself
print_m(matrix); // Prints the matrix
```

## Compile

To compile the test, run ```make``` in the working directory.
