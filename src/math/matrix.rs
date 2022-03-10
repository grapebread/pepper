use std::ops::{Index, IndexMut};
use std::fmt::Display;

pub trait Matrix: Index<(usize, usize)> + IndexMut<(usize, usize)> + Display + Send + Sync {
    type Item;

    fn get(&self, col: usize, row: usize) -> &Self::Item;
    fn set(&mut self, col: usize, row: usize, val: Self::Item);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}
