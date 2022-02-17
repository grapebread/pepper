use std::fmt;
use std::num;

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

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        if x0 > x1 {
            self.draw_line_help(x1, y1, x0, y0, color);
        } else {
            self.draw_line_help(x0, y0, x1, y1, color);
        }
    }

    fn draw_line_help(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        let slope = slope(x0, y0, x1, y1);
        let mut quad = match slope {
            Some(m) => if m > 1_f32 {
                    2
                } else if 0_f32 <= m && m <= 1_f32 {
                    1
                } else if 0_f32 > m && m >= -1_f32 {
                    4
                } else {
                    3
                },
            None => 0,
        };

        let mut x = x0;
        let mut y = y0;

        let dx = 2 * (y1 - y0);
        let dy = 2 * (x0 - x1);
        let mut d = 2 * dx + dy;

        match quad {
            0 => while (y <= y1) {
                self.plot(x, y, color);
                y += 1;
            },
            1 => while (x <= x1) {
                self.plot(x, y, color);

                if (d > 0) {
                    y += 1;
                    d += 2 * dy;
                }

                x += 1;
                d += 2 * dx;
            },
            2 => while (y <= y1) {
                self.plot(x, y, color);

                if (d < 0) {
                    x += 1;
                    d += 2 * dx;
                }

                y += 1;
                d += 2 * dy;
            },
            3 => while (y >= y1) {
                self.plot(x, y, color);

                if (d > 0) {
                    x += 1;
                    d += 2 * dx;
                }

                y -= 1;
                d -= 2 * dy;
            },
            4 => while (x <= x1 && y >= 0) {
                self.plot(x, y, color);

                if (d < 0) {
                    y -= 1;
                    d -= 2 * dy;
                }

                x += 1;
                d += 2 * dx;
            }
            _ => (),
        }
    }

    fn plot(&mut self, x: i32, y: i32, color: Color) {
        self[(x as usize, y as usize)] = color;
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
        &self.raster[idx.0 + (self.height - idx.1 - 1) * self.height]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
        &mut self.raster[idx.0 + (self.height - idx.1 - 1) * self.height]
    }
}

fn slope(x0: i32, y0: i32, x1: i32, y1: i32) -> Option<f32> {
    if x0 == x1 {
        None
    } else {
        Some((y1 - y0) as f32 / (x1 - x0) as f32)
    }
}
