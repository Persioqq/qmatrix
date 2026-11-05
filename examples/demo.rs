use qmatrix::QMatrix;

fn main() {
    println!("Persioqq");

    let mut m1 = QMatrix::new(10, 10);
    m1.print();
    m1.fill_with_c();
    m1.print();

    let mut m2 = QMatrix::new(10, 10);
    m2.print();
    m2.fill_with_d();
    m2.print();

    let res = m1.mul(&m2);
    res.print();

    let res = res.add(&m1);
    res.print();

    let res = res.sub(&m2);
    res.print();

    let res = res.pow(2);
    res.print();
}
