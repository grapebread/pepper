use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use num::{Float, NumCast};

use super::{Matrix, ConstMatrix, Point};

#[derive(Clone)]
pub struct DynMatrix<T> {
    width: usize,
    height: usize,
    pub matrix: Vec<T>,
}

impl<T: Default + Send + Sync + Float> DynMatrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            matrix: vec![T::default(); width * height],
        }
    }

    pub fn identity(width: usize, height: usize) -> Self {
        assert_eq!(width, height, "Matrix is not square");

        let mut ident_matrix = Self::new(width, height);
        let ident_mutex = Arc::new(Mutex::new(&mut ident_matrix));

        (0..width).into_par_iter().for_each(|i| {
            ident_mutex.lock().unwrap()[(i, i)] = NumCast::from(1f64).unwrap();
        });

        ident_matrix
    }

    pub fn add_col(&mut self, m: Point<T>) {
        m.into_iter().for_each(|x| self.matrix.push(x));
        self.width += 1;
    }

    pub fn add_edge(&mut self, p1: Point<T>, p2: Point<T>) {
        self.add_col(p1);
        self.add_col(p2);
    }

    pub fn multiply<const M_WIDTH: usize, const M_HEIGHT: usize>(&mut self, a: &ConstMatrix<T, M_WIDTH, M_HEIGHT>) {
        assert_eq!(M_WIDTH, self.height, "Matrices are not suitable to be multiplied");

        let mut c = DynMatrix::<T>::new(self.width, self.height);

        for i in 0..M_WIDTH {
            for j in 0..self.width {
                for k in 0..self.height {
                    c[(j, i)] = c[(j, i)] + (a[(k, i)] * self[(j, k)]);
                }
            }
        }

        for i in 0..(self.width * self.height) {
            self.matrix[i] = c.matrix[i];
        }
    }
}

impl<T: Send + Sync + Display> Matrix for DynMatrix<T> {
    type Item = T;
    
    fn get(&self, col: usize, row: usize) -> &Self::Item {
        &self[(col, row)]
    }

    fn set(&mut self, col: usize, row: usize, val: Self::Item) {
        self[(col, row)] = val;
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default + Copy> Default for DynMatrix<T> {
    fn default() -> Self {
        Self {
            width: 0,
            height: 4,
            matrix: vec![T::default(); 0 * 4],
        }
    }
}

impl<T> Index<(usize, usize)> for DynMatrix<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[idx.0 * self.height + idx.1]
    }
}

impl<T> IndexMut<(usize, usize)> for DynMatrix<T> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[idx.0 * self.height + idx.1]
    }
}

impl<T: Display> Display for DynMatrix<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..self.height {
            for val in self.matrix.iter().skip(i).step_by(self.height) {
                write!(f, "{:.2} ", val)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}
