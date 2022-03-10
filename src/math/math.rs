use std::fmt::Display;

use num::{Float, NumCast};

use super::{Matrix, ConstMatrix};

pub type Point<T> = Vec<T>;

pub fn new_point<T: Float>(x: T, y: T, z: T) -> Point<T> {
    vec![x, y, z, NumCast::from(1).unwrap()]
}

pub enum RotationAxis {
    X,
    Y,
    Z,
}

impl<f64: Default + Copy + Float + Send + Sync + Display, const WIDTH: usize, const HEIGHT: usize> ConstMatrix<f64, WIDTH, HEIGHT> {
    pub fn make_translate(a: f64, b: f64, c: f64) -> Self {
        let mut m = ConstMatrix::<f64, WIDTH, HEIGHT>::identity();
        m.set(3, 0, a);
        m.set(3, 1, b);
        m.set(3, 2, c);

        m
    }

    pub fn make_scale(a: f64, b: f64, c: f64) -> Self {
        let mut m = ConstMatrix::<f64, WIDTH, HEIGHT>::identity();
        m.set(0, 0, a);
        m.set(1, 1, b);
        m.set(2, 2, c);

        m
    }

    pub fn make_rotate(axis: RotationAxis, theta: f64) -> Self {
        let mut m = ConstMatrix::<f64, WIDTH, HEIGHT>::identity();
        let rad = theta.to_radians();

        match axis {
            RotationAxis::X => {
                m.set(1, 1, rad.cos());
                m.set(2, 1, -rad.sin());
                m.set(1, 2, rad.sin());
                m.set(2, 2, rad.cos());
            },
            RotationAxis::Y => {
                m.set(0, 0, rad.cos());
                m.set(2, 0, rad.sin());
                m.set(0, 2, -rad.sin());
                m.set(2, 2, rad.cos());
            },
            RotationAxis::Z => {
                m.set(0, 0, rad.cos());
                m.set(1, 0, -rad.sin());
                m.set(0, 1, rad.sin());
                m.set(1, 1, rad.cos());
            }
        }

        m
    }
}
