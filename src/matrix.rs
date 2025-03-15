use std::ops::{Index, IndexMut, Mul};

#[derive(Debug)]
pub struct Row(Vec<i32>);

#[derive(Debug)]
pub struct Matrix {
    r: i32,
    c: i32,
    e: Vec<Row>,
}

impl Matrix {
    pub fn new(r: i32, c: i32) -> Matrix {
        let mut e = vec![];
        for _ in 0..r {
            e.push(Row(vec![0; c as usize]));
        }
        Matrix { r, c, e }
    }

    pub fn fill<'a, T: Iterator<Item = &'a i32>>(&mut self, mut it: T) {
        for i in 0..self.r * self.c {
            let r = (i / self.r) as usize;
            let c = (i % self.c) as usize;
            self[r][c] = *it.next().unwrap();
        }
    }

    pub fn shape(&self) -> (i32, i32) {
        (self.r, self.c)
    }
}

impl Index<usize> for Matrix {
    type Output = Row;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Index<usize> for Row {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, b: Self) -> Self::Output {
        let a = self;
        let mut c = Matrix::new(a.r, b.c);

        for idx in 0..(a.r * b.c) {
            let i = idx / b.c;
            let j = idx % b.c;

            for k in 0..a.c {
                c[i as usize][j as usize] += a[i as usize][k as usize] * b[k as usize][j as usize];
            }
        }

        c
    }
}
