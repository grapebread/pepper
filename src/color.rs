use std::fmt;

// color(red, green, blue)
#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[macro_export]
macro_rules! new_color {
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

#[allow(dead_code)]
pub const COLOR_BLACK: Color = new_color!(0, 0, 0);
#[allow(dead_code)]
pub const COLOR_WHITE: Color = new_color!(255, 255, 255);
#[allow(dead_code)]
pub const COLOR_RED: Color = new_color!(255, 0, 0);
#[allow(dead_code)]
pub const COLOR_GREEN: Color = new_color!(0, 255, 0);
#[allow(dead_code)]
pub const COLOR_BLUE: Color = new_color!(0, 0, 255);
#[allow(dead_code)]
pub const COLOR_TEAL: Color = new_color!(0, 255, 255);
#[allow(dead_code)]
pub const COLOR_PURPLE: Color = new_color!(255, 0, 255);
#[allow(dead_code)]
pub const COLOR_YELLOW: Color = new_color!(255, 255, 0);
#[allow(dead_code)]
pub const COLOR_PASTEL_YELLOW: Color = new_color!(239, 169, 74);
