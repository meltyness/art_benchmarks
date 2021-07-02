#![feature(test)]
//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

extern crate test;
mod tests;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == c2 {
            panic!("That's not possible!");
        }

        let mixture = [c1, c2];

        if mixture.iter().any(|x| *x == PrimaryColor::Red) {
            if mixture.iter().any(|x| *x == PrimaryColor::Blue) {
                return SecondaryColor::Purple;
            } else if mixture.iter().any(|x| *x == PrimaryColor::Yellow) {
                return SecondaryColor::Orange;
            }
        }
        SecondaryColor::Green
    }

    pub fn mix_match(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        if c1 == c2 {
            panic!("That's not possible!");
        }

        match (c1, c2) {
            (PrimaryColor::Red, other) | (other, PrimaryColor::Red) => match other {
                PrimaryColor::Blue => SecondaryColor::Purple,
                PrimaryColor::Yellow => SecondaryColor::Orange,
                _ => unreachable!(),
            },
            _ => SecondaryColor::Green,
        }
    }

    pub fn mix_cmp(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        use PrimaryColor::*;
        use SecondaryColor::*;

        match if c1 > c2 { [c2, c1] } else { [c1, c2] } {
            [Red, Blue] => Purple,
            [Red, Yellow] => Orange,
            [Yellow, Blue] => Green,
            _ => panic!("That's not possible!"),
        }
    }

    pub fn mix_u8(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let mut mix = 0_u8;
        for c in [c1, c2] {
            match c {
                PrimaryColor::Red => mix |= 0x01,
                PrimaryColor::Blue => mix |= 0x02,
                PrimaryColor::Yellow => mix |= 0x04,
            }
        }

        match mix {
            0x03 => SecondaryColor::Purple,
            0x05 => SecondaryColor::Orange,
            0x06 => SecondaryColor::Green,
            _ => panic!("That's not possible!"),
        }
    }
}
