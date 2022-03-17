use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Result};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;
use num::{Float, NumCast};

use super::Matrix;

#[derive(Clone, Copy)]
pub struct ConstMatrix<T, const WIDTH: usize, const HEIGHT: usize> {
    pub matrix: [[T; HEIGHT]; WIDTH]
}

impl<T: Default + Copy + Float + Send + Sync + Display, const WIDTH: usize, const HEIGHT: usize> ConstMatrix<T, WIDTH, HEIGHT> {
    pub fn from(arr: [[T; HEIGHT]; WIDTH]) -> Self {
        Self {
            matrix: arr
        }
    }

    pub fn identity() -> Self {
        assert_eq!(WIDTH, HEIGHT, "Matrix is not square");

        let mut ident_matrix = Self::default();
        let ident_mutex = Arc::new(Mutex::new(&mut ident_matrix));

        (0..WIDTH).into_par_iter().for_each(|i| {
            ident_mutex.lock().unwrap()[(i, i)] = NumCast::from(1f64).unwrap();
        });

        ident_matrix
    }

    pub fn multiply<const M_WIDTH: usize, const M_HEIGHT: usize>(&mut self, a: &ConstMatrix<T, M_WIDTH, M_HEIGHT>) {
        assert_eq!(M_WIDTH, HEIGHT, "Matrices are not suitable to be multiplied");

        let mut c = ConstMatrix::<T, WIDTH, HEIGHT>::default();

        for i in 0..M_WIDTH {
            for j in 0..WIDTH {
                for k in 0..HEIGHT {
                    c[(j, i)] = c[(j, i)] + (a[(k, i)] * self[(j, k)]);
                }
            }
        }

        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                self[(i, j)] = c[(i, j)];
            }
        }
    }
}

impl<T: Send + Sync + Display, const WIDTH: usize, const HEIGHT: usize> Matrix for ConstMatrix<T, WIDTH, HEIGHT> {
    type Item = T;

    fn get(&self, col: usize, row: usize) -> &Self::Item {
        &self[(col, row)]
    }
    
    fn set(&mut self, col: usize, row: usize, val: T) {
        self[(col, row)] = val;
    }

    fn width(&self) -> usize {
        WIDTH
    }

    fn height(&self) -> usize {
        HEIGHT
    }
}

impl<T: Default + Copy, const WIDTH: usize, const HEIGHT: usize> Default for ConstMatrix<T, WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            matrix: [[T::default(); HEIGHT]; WIDTH]
        }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Index<(usize, usize)> for ConstMatrix<T, WIDTH, HEIGHT> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.matrix[idx.0][idx.1]
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> IndexMut<(usize, usize)> for ConstMatrix<T, WIDTH, HEIGHT> {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[idx.0][idx.1]
    }
}

impl<T: Display, const WIDTH: usize, const HEIGHT: usize> Display for ConstMatrix<T, WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for i in 0..HEIGHT {
            for slice in self.matrix.iter(){
                write!(f, "{:.2} ", slice[i])?;
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}
