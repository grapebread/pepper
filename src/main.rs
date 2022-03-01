mod color;
mod image;
mod matrix;

use std::fs;

use color::*;
use image::Image;
use matrix::{Matrix, ConstMatrix, identity, new_point, const_multi};

fn main() ->  std::io::Result<()>
{
    println!("Matrix Testing");
    println!("--------------\n");

    let mut m2 = Matrix::<f64>::new(0, 4);
    println!("Empty matrix | m2: (0 x 4)");
    println!("{}\n", m2);

    println!("Testing adding edges");
    println!("Adding (1, 2, 3) and (4, 5, 6) | m2:");
    m2.add_point(new_point(1.0, 2.0, 3.0));
    m2.add_point(new_point(4.0, 5.0, 6.0));
    println!("{}\n", m2);

    println!("Testing identity function | m1:");
    let mut m1 = ConstMatrix::<f64, 4, 4>::new();
    identity(&mut m1);
    println!("{}\n", m1);

    println!("Testing identity matrix multiplication | m1 * m2:");
    m2 = const_multi(m1, &m2);
    println!("{}\n", m2);

    println!("Making new matrix | m3:");
    let mut m3 = ConstMatrix::<f64, 4, 4>::new();
    m3.set_col(0, &[1f64, 2f64, 3f64, 1f64]);
    m3.set_col(1, &[4f64, 5f64, 6f64, 1f64]);
    m3.set_col(2, &[7f64, 8f64, 9f64, 1f64]);
    m3.set_col(3, &[10f64, 11f64, 12f64, 1f64]);
    println!("{}\n", m3);

    println!("Testing matrix multiplication | m3 * m2:");
    m2 = const_multi(m3, &m2);
    println!("{}\n", m2);

    println!("Let's draw");

    let height: i32 = 500;
    let width: i32 = 500;

    let mut image = Image::new(width as usize, height as usize, COLOR_BLACK);
    let mut edgelist = Matrix::<f64>::new(0, 4);

    edgelist.add_edge(new_point(50f64, 450f64, 0f64), new_point(100f64, 450f64, 0f64));
    edgelist.add_edge(new_point(50f64, 450f64, 0f64), new_point(50f64, 400f64, 0f64));
    edgelist.add_edge(new_point(100f64, 450f64, 0f64), new_point(100f64, 400f64, 0f64));
    edgelist.add_edge(new_point(100f64, 400f64, 0f64), new_point(50f64, 400f64, 0f64));

    edgelist.add_edge(new_point(200f64, 450f64, 0f64), new_point(250f64, 450f64, 0f64));
    edgelist.add_edge(new_point(200f64, 450f64, 0f64), new_point(200f64, 400f64, 0f64));
    edgelist.add_edge(new_point(250f64, 450f64, 0f64), new_point(250f64, 400f64, 0f64));
    edgelist.add_edge(new_point(250f64, 400f64, 0f64), new_point(200f64, 400f64, 0f64));

    edgelist.add_edge(new_point(150f64, 400f64, 0f64), new_point(130f64, 360f64, 0f64));
    edgelist.add_edge(new_point(150f64, 400f64, 0f64), new_point(170f64, 360f64, 0f64));
    edgelist.add_edge(new_point(130f64, 360f64, 0f64), new_point(170f64, 360f64, 0f64));

    edgelist.add_edge(new_point(100f64, 340f64, 0f64), new_point(200f64, 340f64, 0f64));
    edgelist.add_edge(new_point(100f64, 320f64, 0f64), new_point(200f64, 320f64, 0f64));
    edgelist.add_edge(new_point(100f64, 340f64, 0f64), new_point(100f64, 320f64, 0f64));
    edgelist.add_edge(new_point(200f64, 340f64, 0f64), new_point(200f64, 320f64, 0f64));

    image.draw_lines(&edgelist, COLOR_TEAL);

    fs::write("image.ppm", format!("{}", image))?;

    Ok(())
}
