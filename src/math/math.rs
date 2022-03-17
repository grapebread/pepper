use std::f64::consts::TAU;

use num::{Float, NumCast};

use super::{Matrix, ConstMatrix, DynMatrix};

pub type Point<T> = Vec<T>;

pub fn new_point<T: Float>(x: T, y: T, z: T) -> Point<T> {
    vec![x, y, z, NumCast::from(1).unwrap()]
}

pub enum RotationAxis {
    X,
    Y,
    Z,
}

pub enum Curve {
    HERMITE,
    BEZIER,
}

impl<const WIDTH: usize, const HEIGHT: usize> ConstMatrix<f64, WIDTH, HEIGHT> {
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

impl DynMatrix<f64> {
    pub fn add_circle(&mut self, cx: f64, cy: f64, _cz: f64, r: f64, step: f64) {
        let mut t = 1f64;
        while t > 0f64 {
            let x0 = r * (TAU * t).cos() + cx;
            let y0 = r * (TAU * t).sin() + cy;

            t -= step;

            let x1 = r * (TAU * t).cos() + cx;
            let y1 = r * (TAU * t).sin() + cy;

            let p0 = new_point(x0, y0, 0f64);
            let p1 = new_point(x1, y1, 0f64);

            self.add_edge(p0, p1);
        }
    }

    pub fn add_curve(&mut self, x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, step: f64, curve: Curve) {
        match curve {
            Curve::HERMITE => {
                let inverse_h = ConstMatrix::from([
                    [2.0, -3.0, 0.0, 1.0],
                    [-2.0, 3.0, 0.0, 0.0],
                    [1.0, -2.0, 1.0, 0.0],
                    [1.0, -1.0, 0.0, 0.0],
                ]);

                let mut g_x = ConstMatrix::from([
                    [x0, x1, x2, x3],
                ]);
                g_x.multiply(&inverse_h);

                let mut g_y = ConstMatrix::from([
                    [y0, y1, y2, y3],
                ]);
                g_y.multiply(&inverse_h);

                let mut t = 0f64;
                while t < 1f64 {
                    let x00 = g_x[(0, 0)] * t * t * t + g_x[(0, 1)] * t * t + g_x[(0, 2)] * t + g_x[(0, 3)];
                    let y00 = g_y[(0, 0)] * t * t * t + g_y[(0, 1)] * t * t + g_y[(0, 2)] * t + g_y[(0, 3)];

                    t += step;

                    let x01 = g_x[(0, 0)] * t * t * t + g_x[(0, 1)] * t * t + g_x[(0, 2)] * t + g_x[(0, 3)];
                    let y01 = g_y[(0, 0)] * t * t * t + g_y[(0, 1)] * t * t + g_y[(0, 2)] * t + g_y[(0, 3)];

                    self.add_edge(new_point(x00, y00, 0.0), new_point(x01, y01, 0.0));
                }
            },
            Curve::BEZIER => {
                let inverse_b = ConstMatrix::from([
                    [-1.0, 3.0, -3.0, 1.0],
                    [3.0, -6.0, 3.0, 0.0],
                    [-3.0, 3.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0, 0.0],
                ]);

                let mut g_x = ConstMatrix::from([
                    [x0, x1, x2, x3],
                ]);
                g_x.multiply(&inverse_b);

                let mut g_y = ConstMatrix::from([
                    [y0, y1, y2, y3],
                ]);
                g_y.multiply(&inverse_b);

                let mut t = 0f64;
                while t < 1f64 {
                    let x00 = g_x[(0, 0)] * t * t * t + g_x[(0, 1)] * t * t + g_x[(0, 2)] * t + g_x[(0, 3)];
                    let y00 = g_y[(0, 0)] * t * t * t + g_y[(0, 1)] * t * t + g_y[(0, 2)] * t + g_y[(0, 3)];

                    t += step;

                    let x01 = g_x[(0, 0)] * t * t * t + g_x[(0, 1)] * t * t + g_x[(0, 2)] * t + g_x[(0, 3)];
                    let y01 = g_y[(0, 0)] * t * t * t + g_y[(0, 1)] * t * t + g_y[(0, 2)] * t + g_y[(0, 3)];

                    self.add_edge(new_point(x00, y00, 0.0), new_point(x01, y01, 0.0));
                }
            },
        }
    }
}
