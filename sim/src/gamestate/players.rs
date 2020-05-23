use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

use colored::Colorize;
use colored::Color;
use rand::distributions::{Distribution, Range};
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
        // Q: Is there some clever arithmetic I could do here instead of `match`?
        match self as i16 - against as i16 {
            // Red beats Green, Green beats Blue, Blue beats Red
            0 => 0,
            1 | -2 => 1,
            2 | -1 => -1,
            _ => panic!(
                "Invalid 'PowerType' values: {}, {}",
                self as i16, against as i16
            ),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Power {
    red: ColorPower,
    blue: ColorPower,
    green: ColorPower,
}

impl Power {
    fn randomize(power_range: Range<i8>, mut rng: &mut rand::ThreadRng) -> Self {
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
            "{red}\t{green}\t{blue}",
            red = self.red.pretty_or_empty(Color::Red),
            blue = self.blue.pretty_or_empty(Color::Blue),
            green = self.green.pretty_or_empty(Color::Green),
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorPower(/*XXX TEMP */pub Option<i8>);

impl ColorPower {
    fn pretty_or_empty(self, color: Color) -> String {
        match self.0 {
            Some(_) => self.pretty(color),
            None => "".to_owned(),
        }
    }
    fn pretty(self, color: Color) -> String {
        format!(
            "{}{}{}",
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

// TODO move to separate file
#[derive(Debug)]
pub enum Role {
    Prophet { target: String },
    Traitor,
    // ...etc
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
// This should *only* be constructed for known players. I.e., the existence of a
// `PName` should guarantee the existence of a player with that name.
// Note: this cannot actually be guaranteed as-is, since the active game-state
// is not a singleton.
pub struct PName(String);

impl fmt::Display for PName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<&str> for PName {
    fn eq(&self, s: &&str) -> bool {
        self.0 == *s
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: PName,
    pub team: String, // TODO should be TName
    // Power is only visible with `Player::strength` and modifiable with
    // `Player::lose_power`
    power: Power,
    // Roles are NEVER changed; they are ONLY used to (1) impact player behavior
    // and (2) determine victory conditions
    role: Option<Box<Role>>,
}

impl Player {
    pub fn new(name: String, team: &str) -> Player {
        // Q: Can I avoid re-creating these each time? (Not sure it matters, but still.) Neither
        // object can be `static`.
        let mut rng = rand::thread_rng();
        let power_range: Range<i8> = Range::new(1, 6);
        Player {
            name: PName(name),
            team: team.to_owned(),
            power: Power::randomize(power_range, &mut rng),
            role: None,
        }
    }

    pub fn strength(&self, ptype: PowerType) -> i8 {
        self.power[ptype].into()
    }

    pub fn lose_power(&mut self, ptype: PowerType) {
        self.power[ptype] = ColorPower(None)
    }

    pub fn pretty<'a>(players: impl Iterator<Item=&'a Player>) -> String {
        let mut formatted = String::new();
        for player in players {
            // Newlines are added by the `Player` formatter.
            formatted = format!("{}{}", &formatted, player);
        }
        formatted
    }
}

// Does not print the team name or the role.
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}:\t\t{}", self.name.0, self.power)
    }
}

#[derive(Debug, Default)]
pub struct PlayersByName(HashMap<PName, Player>);

impl PlayersByName {
    pub fn from(team: &str, names: impl Iterator<Item=String>) -> Self {
        let mut map = HashMap::new();
        for name in names {
            map.insert(PName(name.clone()), Player::new(name, team));
        }
        PlayersByName(map)
    }
    pub fn find_player(&self, name: &PName) -> Option<&Player> {
        self.0.get(name)
    }
    /* TODO - do I need this?
    pub fn find_mut(&mut self, name: &PName) -> Option<&mut Player> {
        self.0.get_mut(name)
    }
    */
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.0.values()
    }
    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0.values_mut()
    }
}
