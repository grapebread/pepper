mod color;
mod image;
mod matrix;

use std::fs;

use color::*;
use image::Image;
use matrix::{Matrix, ConstMatrix, Point, identity, new_point};

fn main() ->  std::io::Result<()>
{
    println!("Matrix Testing");
    println!("--------------\n");

    let mut m2 = Matrix::<f64>::new(0, 4);
    println!("Empty matrix");
    println!("{}\n", m2);

    println!("Testing adding edges");
    println!("Adding (1, 2, 3) and (4, 5, 6)");
    m2.add(new_point(1.0, 2.0, 3.0));
    m2.add(new_point(4.0, 5.0, 6.0));
    println!("{}\n", m2);

    println!("Testing identity function");
    let mut m1 = ConstMatrix::<f64, 3, 3>::new();
    identity(&mut m1);
    println!("{}", m1);

    Ok(())
}
