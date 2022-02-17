mod color;
mod image;

use std::fs;

use image::Image;
use color::*;

fn main() ->  std::io::Result<()>
{
    let height: i32 = 500;
    let width: i32 = 500;
    let mut image = Image::new(width as usize, height as usize, COLOR_BLACK);
    let mut color = new!(0, 255, 0);
    
    // octants 1 and 5
    image.draw_line(0, 0, width - 1, height - 1, color);
    image.draw_line(0, 0, width - 1, height / 2, color);
    image.draw_line(width - 1, height - 1, 0, height / 2, color);
    println!();
    // (499, 499, 0, 250)

    // octants 4 and 8
    color.blue = 255;
    image.draw_line(0, height - 1, width - 1, 0, color);
    image.draw_line(0, height - 1, width - 1, height / 2, color);
    image.draw_line(width - 1, 0, 0, height / 2, color);
    println!();
    // (499, 0, 0, 250)
    
    // octants 2 and 6
    color.red = 255;
    color.green = 0;
    color.blue = 0;
    image.draw_line(0, 0, width / 2, height - 1, color);
    image.draw_line(width - 1, height - 1, width / 2, 0, color);
    // (499, 499, 250, 0)
    image.draw_line(width / 2, 0, width - 1, height - 1, color);
    println!();

    // octants 3 and 7
    color.blue = 255;
    image.draw_line(0, height - 1, width / 2, 0, color);
    image.draw_line(width - 1, 0, width / 2, height - 1, color);
    println!();
    // (499, 0, 250, 499)

    // horizontal and vertical
    color.blue = 0;
    color.green = 255;
    image.draw_line(0, height / 2, width - 1, height / 2, color);
    image.draw_line(width / 2, 0, width / 2, height - 1, color);
    println!();

    fs::write("image.ppm", format!("{}", image))?;

    Ok(())
}
