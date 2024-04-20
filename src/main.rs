use regex::Regex;
use std::fmt::Display;
use std::str::FromStr;
use std::u8;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
} // TODO: design data structure
#[derive(Debug, PartialEq, Eq)]
struct ParseError;
trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn b(&self) -> u8 {
        self.b
    }
    fn g(&self) -> u8 {
        self.g
    }
    fn r(&self) -> u8 {
        self.r
    }
}

impl FromStr for Rgb {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new("^#([a-f0-9]{6})$").unwrap();
        if !pattern.is_match(s) {
            Err(ParseError)
        } else {
            let strip_string = s.trim_start_matches("#");
            let r_val = u8::from_str_radix(&strip_string[0..2], 16).unwrap();
            let g_val = u8::from_str_radix(&strip_string[2..4], 16).unwrap();
            let b_val = u8::from_str_radix(&strip_string[4..6], 16).unwrap();
            let channels = Rgb {
                r: r_val,
                b: g_val,
                g: b_val,
            };
            return Ok(channels);
        }
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
