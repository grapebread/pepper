use std::fmt;

// color(red, green, blue)
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[macro_export]
macro_rules! new {
    ($red:expr, $green:expr, $blue:expr) => {
        Color {
            red: $red, 
            green: $green, 
            blue: $blue,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.red, self.green, self.blue)
    }
}

pub const COLOR_BLACK: Color = new!(0, 0, 0);
pub const COLOR_WHITE: Color = new!(255, 255, 255);
pub const COLOR_RED: Color = new!(255, 0, 0);
pub const COLOR_GREEN: Color = new!(0, 255, 0);
pub const COLOR_BLUE: Color = new!(0, 0, 255);
