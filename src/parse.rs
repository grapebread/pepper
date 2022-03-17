use std::fs::{File, write};
use std::io::{self, prelude::*, BufReader};

use scan_fmt::scan_fmt;

use crate::color::{COLOR_BLACK, COLOR_TEAL};
use crate::image::Image;
use crate::math::{ConstMatrix, DynMatrix, new_point, RotationAxis, Curve};

pub fn parse<const WIDTH: usize, const HEIGHT: usize>(transform: &mut ConstMatrix<f64, WIDTH, HEIGHT>, edgelist: &mut DynMatrix<f64>, image: &mut Image, filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        match line.unwrap().as_str() {
            "line" => {
                let data = lines.next().unwrap().unwrap();
                let (x0, y0, z0, x1, y1, z1) = scan_fmt!(data.as_str(), "{} {} {} {} {} {}", f64, f64, f64, f64, f64, f64).expect("Unable to read line data");
                edgelist.add_edge(new_point(x0, y0, z0), new_point(x1, y1, z1));
            },
            "ident" => {
                transform.matrix = ConstMatrix::<f64, WIDTH, HEIGHT>::identity().matrix;
            },
            "scale" => {
                let data = lines.next().unwrap().unwrap();
                let (sx, sy, sz) = scan_fmt!(data.as_str(), "{} {} {}", f64, f64, f64).expect("Unable to read scale data");
                let sm = ConstMatrix::<f64, WIDTH, HEIGHT>::make_scale(sx, sy, sz);
                transform.multiply(&sm);
            },
            "move" => {
                let data = lines.next().unwrap().unwrap();
                let (tx, ty, tz) = scan_fmt!(data.as_str(), "{} {} {}", f64, f64, f64).expect("Unable to read translate data");
                let tm = ConstMatrix::<f64, WIDTH, HEIGHT>::make_translate(tx, ty, tz);
                transform.multiply(&tm);
            },
            "rotate" => {
                let data = lines.next().unwrap().unwrap();
                let (axis, theta) = scan_fmt!(data.as_str(), "{} {}", String, f64).expect("Unable to read translate data");

                match axis.as_str() {
                    "x" => {
                        let rm = ConstMatrix::<f64, WIDTH, HEIGHT>::make_rotate(RotationAxis::X, theta);
                        transform.multiply(&rm);
                    },
                    "y" => {
                        let rm = ConstMatrix::<f64, WIDTH, HEIGHT>::make_rotate(RotationAxis::Y, theta);
                        transform.multiply(&rm);
                    },
                    "z" => {
                        let rm = ConstMatrix::<f64, WIDTH, HEIGHT>::make_rotate(RotationAxis::Z, theta);
                        transform.multiply(&rm);
                    }
                    _ => ()
                }
            },
            "circle" => {
                let data = lines.next().unwrap().unwrap();
                let (cx, cy, cz, r) = scan_fmt!(data.as_str(), "{} {} {} {}", f64, f64, f64, f64).expect("Unable to read circle data");
                edgelist.add_circle(cx, cy, cz, r, 0.005);
            },
            "hermite" => {
                let data = lines.next().unwrap().unwrap();
                let (x0, y0, x1, y1, rx0, ry0, rx1, ry1) = scan_fmt!(data.as_str(), "{} {} {} {} {} {} {} {}", f64, f64, f64, f64, f64, f64, f64, f64).expect("Unable to read hermite data");
                edgelist.add_curve(x0, y0, x1, y1, rx0, ry0, rx1, ry1, 0.005, Curve::HERMITE);
            }
            "bezier" => {
                let data = lines.next().unwrap().unwrap();
                let (x0, y0, x1, y1, x2, y2, x3, y3) = scan_fmt!(data.as_str(), "{} {} {} {} {} {} {} {}", f64, f64, f64, f64, f64, f64, f64, f64).expect("Unable to read hermite data");
                edgelist.add_curve(x0, y0, x1, y1, x2, y2, x3, y3, 0.005, Curve::BEZIER);
            }
            "apply" => {
                edgelist.multiply(&transform);
            },
            "display" => {
                // can't display anything on wsl
            },
            "save" => {
                let data = lines.next().unwrap().unwrap();
                let save_name = scan_fmt!(data.as_str(), "{}", String).expect("Unable to read save filename");
                image.reset(COLOR_BLACK);
                image.draw_lines(edgelist, COLOR_TEAL);
                write(save_name, format!("{}", image))?;
            }
            unknown => {
                match unknown.chars().next().unwrap() {
                    '#' => (),
                    _ => {
                        println!("Feature not implemented: {}", unknown);
                    }
                }
            }
        }
    }

    Ok(())
}
