
use std::error::Error;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

struct hex_color {
    r: u8,
    g: u8,
    b: u8,
}

impl hex_color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl fmt::Display for hex_color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

impl FromStr for hex_color {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (
            s.chars().nth(0),
            s.chars().skip(1).take(2).collect::<String>(),
            s.chars().skip(3).take(2).collect::<String>(),
            s.chars().skip(5).take(2).collect::<String>(),
            s.chars().skip(7).nth(0),
        ) {
            (Some('#'), r, g, b, None) => Ok(Self::new(
                u8::from_str_radix(&r, 16)?,
                u8::from_str_radix(&g, 16)?,
                u8::from_str_radix(&b, 16)?,
            )),
            (_, _, _, _, _) => Err("Cannot convert string to HexColour".into()),
        }
    }
}

impl FromIterator<hex_color> for hex_color {
    fn from_iter<I: IntoIterator<Item = hex_color>>(iter: I) -> Self {
        let (count, (red, green, blue)) =
            iter.into_iter()
                .fold((0_usize, (0_f32, 0_f32, 0_f32)), |(c, (red, green, blue)), h| {
                    (
                        c + 1,
                        (
                            red + f32::from(h.r),
                            green + f32::from(h.g),
                            blue + f32::from(h.b),
                        ),
                    )
                });
        Self {
            r: (red / count as f32) as u8,
            g: (green / count as f32) as u8,
            b: (blue / count as f32) as u8,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    for hexcolor in &[
        hex_color::new(219, 29, 64),
        hex_color::new(108, 77, 255),
        hex_color::new(14, 195, 10),
        hex_color::new(47, 0, 0),
        hex_color::new(0, 47, 0),
        hex_color::new(0, 0, 47),
        hex_color::new(0, 0, 0),
        hex_color::new(255, 255, 255),
    ] {
        println!("{}", hexcolor);
    }

    Ok(())
}
