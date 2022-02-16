use std::fmt;

use crate::color::*;

pub struct Image {
    height: usize,
    width: usize,
    raster: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize, color: Color) -> Image {
        let raster = vec![color; width * height];
        Image {
            width,
            height,
            raster,
        }
    }

    pub fn draw_line(&mut self, x0: isize, y0: isize, x1: isize, y1: isize, color: Color) {
        let slope = (y1 - y0) as f32 / (x1 - x0) as f32;
        println!("slope: {}", slope);
        let mut x = x0;
        let mut y = y0;
        let a = 2 * (y1 - y0);
        let b = -2 * (x1 - x0);
        let mut d = 2 * a + b;

        if slope > 0_f32 {
            while x <= x1 {
                self.plot(x as usize, y as usize, color);
                if d > 0 {
                    y += 1;
                    d += 2 * b;
                }
                x += 1;
                d += 2 * a;
            }
        } else {
            while x <= x1 {
                self.plot(x as usize, y as usize, color);
                if d < 0 {
                    y += 1;
                    d += 2 * b;
                }
                x += 1;
                d += 2 * a;
            }
        }
    }

    /*
    fn draw_vert_line(&mut self, x: usize, y0: usize, y1: usize, color: Color) {
        if y1 > y0 {
            for y in y0..y1 {
                self.plot(x, y, color);
            }
        } else {
            for y in y1..y0 {
                self.plot(x, y, color);
            }
        }
    }

    pub fn draw_hori_line(&mut self, x0: usize, x1: usize, y: usize, color: Color) {
        if x1 > x0 {
            for x in x0..x1 {
                self.plot(x, y, color);
            }
        } else {
            for x in x1..x0 {
                self.plot(x, y, color);
            }
        }
    }
    */

    fn plot(&mut self, x: usize, y: usize, color: Color) {
        self[(x, y)] = color;
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;
        self.raster.iter().try_for_each(|color| write!(f, "{} ", color))?;
        Ok(())
    }
}

impl std::ops::Index<(usize, usize)> for Image {
    type Output = Color;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.raster[idx.0 + idx.1 * self.height]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.raster[idx.0 + idx.1 * self.height]
    }
}
