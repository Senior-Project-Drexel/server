use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Debug)]
pub struct Row<T: Copy + Clone + Default>(Vec<T>);

#[derive(Debug)]
pub struct Matrix<T: Copy + Clone + Default> {
    r: u32,
    c: u32,
    e: Vec<Row<T>>,
    i: u32
}

impl <'a, T: 'a + Copy + Clone + Default> Matrix<T> {
    pub fn new(r: u32, c: u32) -> Self {
        let mut e = vec![];
        for _ in 0..r {
            e.push(Row(vec![T::default(); c as usize]));
        }
        Matrix { r, c, e, i: 0 }
    }

    pub fn fill<IT: Iterator<Item = &'a T>>(&mut self, mut it: IT) {
        for i in 0..self.r * self.c {
            let r = (i / self.r) as usize;
            let c = (i % self.c) as usize;
            self[r][c] = *it.next().unwrap();
        }
    }

    pub fn shape(&self) -> (u32, u32) {
        (self.r, self.c)
    }
}

impl <T: Copy + Clone + Default> Index<usize> for Matrix<T> {
    type Output = Row<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl <T: Copy + Clone + Default> IndexMut<usize> for Matrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl <T: Copy + Clone + Default> Index<usize> for Row<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl <T: Copy + Clone + Default> IndexMut<usize> for Row<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl <T: Copy + Clone + Default + Mul<T, Output = T> + Add<Output = T>> Mul for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, b: Self) -> Self::Output {
        let a = self;
        let mut c = Matrix::new(a.r, b.c);

        for idx in 0..(a.r * b.c) {
            let i = idx / b.c;
            let j = idx % b.c;

            for k in 0..a.c {
                c[i as usize][j as usize] = c[i as usize][j as usize] + a[i as usize][k as usize] * b[k as usize][j as usize];
            }
        }

        c
    }
}

impl <T: Copy + Clone + Default> Iterator for Matrix<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.r * self.c {
            return None;
        }
        let e = self[(self.i / self.c) as usize][(self.i % self.c) as usize];
        self.i += 1;
        Some(e)
    }
}