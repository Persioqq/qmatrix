use std::process::exit;

pub struct QMatrix {
    pub l: usize,
    pub c: usize,
    pub m: Vec<Vec<i64>>,
}

impl QMatrix {
    pub fn new(l: usize, c: usize) -> Self {
        QMatrix {
            l,
            c,
            m: vec![vec![0; c]; l],
        }
    }

    pub fn print(&self) {
        print!("\n{}x{} qmatrix:\n\n", self.l, self.c);

        for i in 0..self.l {
            print!("|");
            for j in 0..self.c {
                print!(" {} \t", self.m[i][j]);
            }
            print!("\t|\n");
        }
    }

    pub fn fill_with(&mut self, n: i64) {
        for i in 0..self.l {
            for j in 0..self.c {
                self.m[i][j] = n;
            }
        }
    }

    pub fn fill_with_c(&mut self) {
        let mut n = 1;

        for i in 0..self.l {
            for j in 0..self.c {
                self.m[i][j] = n;
                n += 1;
            }
        }
    }

    pub fn fill_with_d(&mut self) {
        let mut n: i64 = (self.l * self.c).try_into().unwrap();

        for i in 0..self.l {
            for j in 0..self.c {
                self.m[i][j] = n;
                n -= 1;
            }
        }
    }

    pub fn add(&self, other: &QMatrix) -> QMatrix {
        if self.l != other.l || self.c != other.c {
            println!("wrong qmatrix size");
            exit(1);
        }

        let mut res = QMatrix::new(self.l, self.c);

        for i in 0..self.l {
            for j in 0..self.c {
                res.m[i][j] = self.m[i][j] + other.m[i][j];
            }
        }

        res
    }

    pub fn sub(&self, other: &QMatrix) -> QMatrix {
        if self.l != other.l || self.c != other.c {
            println!("wrong qmatrix size");
            exit(1);
        }

        let mut res = QMatrix::new(self.l, self.c);

        for i in 0..self.l {
            for j in 0..self.c {
                res.m[i][j] = self.m[i][j] - other.m[i][j];
            }
        }

        res
    }

    pub fn mul(&self, other: &QMatrix) -> QMatrix {
        if self.c != other.l {
            println!("wrong qmatrix size");
            exit(1);
        }

        let mut res = QMatrix::new(self.l, other.c);

        for i in 0..self.l {
            for j in 0..other.c {
                for k in 0..other.l {
                    res.m[i][j] += self.m[i][k] * other.m[k][j];
                }
            }
        }

        res
    }

    pub fn pow(&self, n: f64) -> QMatrix {
        let mut res = QMatrix::new(self.l, self.c);

        for i in 0..self.l {
            for j in 0..self.c {
                res.m[i][j] = (self.m[i][j] as f64).powf(n) as i64;
            }
        }

        res
    }
}
