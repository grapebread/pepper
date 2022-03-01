use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct ConstMatrix<T, const WIDTH: usize, const HEIGHT: usize> {
    arr: [[T; HEIGHT]; WIDTH]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> ConstMatrix<T, WIDTH, HEIGHT> where T: Default + Copy {
    pub fn new() -> Self {
        Self {
            arr: [[T::default(); HEIGHT]; WIDTH]
        }
    }

    pub fn set_col(&mut self, col_num: usize, col: &[T; HEIGHT]) {
        self.arr[col_num] = col.clone();
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for ConstMatrix<T, WIDTH, HEIGHT> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.arr[idx.0][idx.1]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)> for ConstMatrix<T, WIDTH, HEIGHT> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[idx.0][idx.1]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Display for ConstMatrix<T, WIDTH, HEIGHT> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..HEIGHT {
            for slice in self.arr.iter() {
                write!(f, "{:.2} ", slice[i])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub type Point<T> = Vec<T>;

#[derive(Clone)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    pub arr: Vec<Point<T>>,
}

impl<T> Matrix<T> where T: Default + Copy {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            arr: vec![vec![T::default(); height]; width],
        }
    }

    pub fn set_inner(&mut self, x: usize, p: Point<T>) {
        self.arr[x] = p;
    }

    pub fn add_point(&mut self, p: Point<T>) {
        self.arr.push(p);
        self.width += 1;
    }

    pub fn add_edge(&mut self, p1: Point<T>, p2: Point<T>) {
        self.add_point(p1);
        self.add_point(p2);
    }
}

pub fn const_multi<const WIDTH: usize, const HEIGHT: usize>(a: ConstMatrix<f64, WIDTH, HEIGHT>, b: &Matrix<f64>) -> Matrix<f64> {
    let mut c = Matrix::<f64>::new(b.width, b.height);

    for m in 0..WIDTH {
        for r in 0..b.width {
            for k in 0..b.height {
                c[(r, m)] += a[(k, m)] * b[(r, k)];
            }
        }
    }

    c
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.arr[idx.0][idx.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.arr[idx.0][idx.1]
    }
}

impl<T> Display for Matrix<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..self.height {
            for slice in self.arr.iter() {
                write!(f, "{:.2} ", slice[i])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub fn identity<const WIDTH: usize, const HEIGHT: usize>(array: &mut ConstMatrix<f64, WIDTH, HEIGHT>) {
    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            if i == j {
                array[(i, j)] = 1_f64;
            } else {
                array[(i, j)] = 0_f64;
            }
        }
    }
}

pub fn new_point(x: f64, y: f64, z: f64) -> Point<f64> {
    vec![x, y, z, 1_f64]
}
