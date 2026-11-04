use qmatrix::*;

fn main() {
    println!("Persioqq");

    let mut m1 = new_m(10, 10);
    print_m(&m1);
    fill_m_with_c(&mut m1);
    print_m(&m1);

    let mut m2 = new_m(10, 10);
    print_m(&m2);
    fill_m_with_d(&mut m2);
    print_m(&m2);

    let res = mul_m(&m1, &m2);
    print_m(&res);

    let res = add_m(&res, &m1);
    print_m(&res);

    let res = sub_m(&res, &m2);
    print_m(&res);

    let res = pow_m(&res, 2.0);
    print_m(&res);
}
