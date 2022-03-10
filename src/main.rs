use pepper::color::COLOR_BLACK;
use pepper::image::Image;
use pepper::math::{ConstMatrix, DynMatrix};
use pepper::parse::parse;

fn main() ->  std::io::Result<()>
{
    let width = 500;
    let height = 500;

    let mut edgelist = DynMatrix::<f64>::new(0, 4);
    let mut transform = ConstMatrix::<f64, 4, 4>::default();
    let mut image = Image::new(width, height, COLOR_BLACK);

    parse(&mut transform, &mut edgelist, &mut image, "script")?;

    Ok(())
}
