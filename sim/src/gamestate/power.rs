use std::fmt;
use std::ops::{Index, IndexMut};

use colored::Color;
use colored::Colorize;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use rand_derive::Rand;

#[derive(Copy, Clone, Debug, Rand)]
pub enum PowerType {
    Red = 0,
    Blue = 1,
    Green = 2,
}

impl From<PowerType> for Color {
    fn from(pt: PowerType) -> Self {
        match pt {
            PowerType::Red => Color::Red,
            PowerType::Blue => Color::Blue,
            PowerType::Green => Color::Green,
        }
    }
}

impl fmt::Display for PowerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).color(Color::from(*self)))
    }
}

// TODO DESIGN relative color advantage?
const POWER_ADVANTAGE_MULTIPLYER: i16 = 2;

impl PowerType {
    // May be negative
    pub fn relative_advantage(self, against: PowerType) -> i16 {
        self.unit_advantage(against) * POWER_ADVANTAGE_MULTIPLYER
    }

    fn unit_advantage(self, against: PowerType) -> i16 {
        /*
        Red beats Green, Green beats Blue, Blue beats Red:
        (self - against):
        0 => 0,
        1 | -2 => 1,
        2 | -1 => -1,

        This is modular arithmetic, rotated by 1 and back again; add another
        3 to make the values positive so that `%` performs the correct
        operation.
        */
        (self as i16 - against as i16 + 4) % 3 - 1
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Power {
    red: ColorPower,
    blue: ColorPower,
    green: ColorPower,
}

impl Power {
    pub fn randomize(power_range: Uniform<i8>, mut rng: &mut ThreadRng) -> Self {
        Power {
            red: ColorPower(Some(power_range.sample(&mut rng))),
            green: ColorPower(Some(power_range.sample(&mut rng))),
            blue: ColorPower(Some(power_range.sample(&mut rng))),
        }
    }
}

impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            // Q: the width argument only seems to work for the 'empty' case. Why?
            "{:5}{:5}{}",
            self.red.pretty_or_empty(Color::Red),
            self.blue.pretty_or_empty(Color::Blue),
            self.green.pretty_or_empty(Color::Green),
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorPower(Option<i8>);

impl ColorPower {
    pub fn discard(&mut self) {
        self.0 = None;
    }

    // TODO better name
    pub fn nonempty(self) -> bool {
        self.0.is_some()
    }

    fn pretty_or_empty(self, color: Color) -> String {
        match self.0 {
            Some(_) => self.pretty(color),
            None => "".to_owned(),
        }
    }
    fn pretty(self, color: Color) -> String {
        format!(
            "{}{}{:3}", // Q: width works here but not above (in Power::fmt); why?
            "(".to_string().bold().color(color),
            self.to_string(),
            ")".to_string().bold().color(color),
        )
    }
}

impl fmt::Display for ColorPower {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", i8::from(*self).to_string().white().bold())
    }
}

impl From<ColorPower> for i8 {
    fn from(cp: ColorPower) -> Self {
        cp.0.unwrap_or(0)
    }
}

impl Index<PowerType> for Power {
    type Output = ColorPower;
    fn index(&self, ptype: PowerType) -> &ColorPower {
        match ptype {
            PowerType::Red => &self.red,
            PowerType::Blue => &self.blue,
            PowerType::Green => &self.green,
        }
    }
}

impl IndexMut<PowerType> for Power {
    fn index_mut(&mut self, ptype: PowerType) -> &mut Self::Output {
        match ptype {
            PowerType::Red => &mut self.red,
            PowerType::Blue => &mut self.blue,
            PowerType::Green => &mut self.green,
        }
    }
}

#[test]
fn power_advantage() {
    assert!(PowerType::Red.relative_advantage(PowerType::Red) == 0);
    assert!(PowerType::Red.relative_advantage(PowerType::Green) == 2);
    assert!(PowerType::Red.relative_advantage(PowerType::Blue) == -2);
    assert!(PowerType::Green.relative_advantage(PowerType::Red) == -2);
    assert!(PowerType::Green.relative_advantage(PowerType::Green) == 0);
    assert!(PowerType::Green.relative_advantage(PowerType::Blue) == 2);
    assert!(PowerType::Blue.relative_advantage(PowerType::Red) == 2);
    assert!(PowerType::Blue.relative_advantage(PowerType::Green) == -2);
    assert!(PowerType::Blue.relative_advantage(PowerType::Blue) == 0);
}
