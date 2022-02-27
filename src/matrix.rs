use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub struct ConstMatrix<T, const WIDTH: usize, const HEIGHT: usize> {
    arr: [[T; WIDTH]; HEIGHT]
}

impl<T, const WIDTH: usize, const HEIGHT: usize> ConstMatrix<T, WIDTH, HEIGHT> where T: Default + Copy {
    pub fn new() -> Self {
        Self {
            arr: [[T::default(); WIDTH]; HEIGHT]
        }
    }

    pub fn fill(&mut self, n: T) {
        self.arr = [[n; WIDTH]; HEIGHT];
    }

    pub fn set(&mut self, x: usize, y: usize, n: T) {
        self.arr[x][y] = n;
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

    pub fn set_val(&mut self, x: usize, y: usize, n: T) {
        self[(x, y)] = n;
    }

    pub fn set_inner(&mut self, x: usize, p: Point<T>) {
        self.arr[x] = p;
    }

    pub fn add(&mut self, p: Point<T>) {
        self.arr.push(p);
        self.width += 1;
    }

    pub fn const_multi<const WIDTH: usize, const HEIGHT: usize>(&mut self, m: ConstMatrix<T, WIDTH, HEIGHT>) {
        
    }
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
