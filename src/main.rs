mod color;
mod image;

use std::fs;

use image::Image;
use color::{COLOR_BLACK, COLOR_BLUE, COLOR_RED, COLOR_WHITE};

fn main() ->  std::io::Result<()>
{
    let height = 500;
    let width = 500;
    let mut image = Image::new(width, height, COLOR_BLACK);
    
    image.draw_line(50, 200, 200, 50, COLOR_RED);
    //image.draw_line(50, 50, 4, 499, COLOR_WHITE);

    fs::write("image.ppm", format!("{}", image))?;

    Ok(())
}
