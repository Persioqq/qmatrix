
use std::process::exit;
 
pub struct QMatrix {
    pub l: usize,
    pub c: usize,
    pub m: Vec<Vec<i64>>,
}
 
pub fn new_m(l: usize, c: usize) -> QMatrix {
    QMatrix {
        l,
        c,
        m: vec![vec![0; c]; l],
    }
}
 
pub fn print_m(m: &QMatrix) {
    print!("\n{}x{} qmatrix:\n\n", m.l, m.c);
 
    for i in 0..m.l {
        print!("|");
        for j in 0..m.c {
            print!(" {} \t", m.m[i][j]);
        }
        print!("\t|\n");
    }
}
 
pub fn fill_m_with(m: &mut QMatrix, n: i64) {
    for i in 0..m.l {
        for j in 0..m.c {
            m.m[i][j] = n;
        }
    }
}
 
pub fn fill_m_with_c(m: &mut QMatrix) {
    let mut n = 1;
 
    for i in 0..m.l {
        for j in 0..m.c {
            m.m[i][j] = n;
            n += 1;
        }
    }
}

pub fn fill_m_with_d(m: &mut QMatrix) {
    let mut n: i64 = (m.l * m.c).try_into().unwrap();
 
    for i in 0..m.l {
        for j in 0..m.c {
            m.m[i][j] = n;
            n -= 1;
        }
    }
}

pub fn add_m(m1: &QMatrix, m2: &QMatrix) -> QMatrix {
    if m1.l != m2.l || m1.c != m2.c {
        println!("wrong qmatrix size");
        exit(1);
    }
 
    let mut res = new_m(m1.l, m1.c);
 
    for i in 0..m1.l {
        for j in 0..m1.c {
            res.m[i][j] = m1.m[i][j] + m2.m[i][j];
        }
    }
 
    res
}
 
pub fn sub_m(m1: &QMatrix, m2: &QMatrix) -> QMatrix {
    if m1.l != m2.l || m1.c != m2.c {
        println!("wrong qmatrix size");
        exit(1);
    }
 
    let mut res = new_m(m1.l, m1.c);
 
    for i in 0..m1.l {
        for j in 0..m1.c {
            res.m[i][j] = m1.m[i][j] - m2.m[i][j];
        }
    }
 
    res
}
 
pub fn mul_m(m1: &QMatrix, m2: &QMatrix) -> QMatrix {
    if m1.c != m2.l {
        println!("wrong qmatrix size");
        exit(1);
    }
 
    let mut res = new_m(m1.l, m2.c);
 
    for i in 0..m1.l {
        for j in 0..m2.c {
            for k in 0..m2.l {
                res.m[i][j] += m1.m[i][k] * m2.m[k][j];
            }
        }
    }
 
    res
}
 
pub fn pow_m(m: &QMatrix, n: f64) -> QMatrix {
    let mut res = new_m(m.l, m.c);
 
    for i in 0..m.l {
        for j in 0..m.c {
            res.m[i][j] = (m.m[i][j] as f64).powf(n) as i64;
        }
    }
 
    res
}

