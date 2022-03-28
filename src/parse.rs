use std::fs::{File, write};
use std::io::{self, prelude::*, BufReader};

use scan_fmt::scan_fmt;

use crate::color::{COLOR_BLACK, COLOR_TEAL, COLOR_PASTEL_YELLOW};
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
                edgelist.add_edge(&new_point(x0, y0, z0), &new_point(x1, y1, z1));
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
                edgelist.add_circle(cx, cy, cz, r, 0.05);
            },
            "hermite" => {
                let data = lines.next().unwrap().unwrap();
                let (x0, y0, x1, y1, rx0, ry0, rx1, ry1) = scan_fmt!(data.as_str(), "{} {} {} {} {} {} {} {}", f64, f64, f64, f64, f64, f64, f64, f64).expect("Unable to read hermite data");
                edgelist.add_curve(x0, y0, x1, y1, rx0, ry0, rx1, ry1, 0.05, Curve::HERMITE);
            }
            "bezier" => {
                let data = lines.next().unwrap().unwrap();
                let (x0, y0, x1, y1, x2, y2, x3, y3) = scan_fmt!(data.as_str(), "{} {} {} {} {} {} {} {}", f64, f64, f64, f64, f64, f64, f64, f64).expect("Unable to read hermite data");
                edgelist.add_curve(x0, y0, x1, y1, x2, y2, x3, y3, 0.05, Curve::BEZIER);
            }
            "clear" => {
                edgelist.matrix.clear();
            }
            "box" => {
                let data = lines.next().unwrap().unwrap();
                let (x, y, z, width, height, depth) = scan_fmt!(data.as_str(), "{} {} {} {} {} {}", f64, f64, f64, f64, f64, f64).expect("Unable to read box data");
                edgelist.add_box(new_point(x, y, z), width, height, depth);
            }
            "sphere" => {
                let data = lines.next().unwrap().unwrap();
                let (cx, cy, cz, radius) = scan_fmt!(data.as_str(), "{} {} {} {}", f64, f64, f64, f64).expect("Unable to read sphere data");
                edgelist.add_sphere(new_point(cx, cy, cz), radius, 0.05);
            }
            "torus" => {
                let data = lines.next().unwrap().unwrap();
                let (cx, cy, cz, r0, r1) = scan_fmt!(data.as_str(), "{} {} {} {} {}", f64, f64, f64, f64, f64).expect("Unable to read sphere data");
                edgelist.add_torus(new_point(cx, cy, cz), r0, r1, 0.06)
            }
            "apply" => {
                edgelist.multiply(&transform);
            },
            "display" => {
                println!("Unable to display anything on my pc due to using WSL");
            },
            "save" => {
                let data = lines.next().unwrap().unwrap();
                let save_name = scan_fmt!(data.as_str(), "{}", String).expect("Unable to read save filename");
                image.reset(COLOR_BLACK);
                image.draw_lines(edgelist, COLOR_PASTEL_YELLOW);
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
