use std::collections::HashMap;
use std::fmt;
use std::ops::{Index, IndexMut};

use colored::Colorize;
use rand::distributions::{Distribution, Range};
use rand_derive::Rand;

#[derive(Copy, Clone, Debug, Rand)]
pub enum PowerType {
    Red = 0,
    Blue = 1,
    Green = 2,
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
            "{redstart}{red}{redend}, {greenstart}{green}{greenend}, {bluestart}{blue}{blueend}",
            redstart = "(".to_string().bold().red(),
            red = self.red.to_string().on_red(),
            redend = ")".to_string().bold().red(),
            greenstart = "(".to_string().bold().green(),
            green = self.green.to_string().on_green(),
            greenend = ")".to_string().bold().green(),
            bluestart = "(".to_string().bold().blue(),
            blue = self.blue.to_string().on_blue(),
            blueend = ")".to_string().bold().blue(),
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorPower(/*XXX TEMP */pub Option<i8>);

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
    pub fn new(name: &str, team: &str) -> Player {
        // Q: Can I avoid re-creating these each time? (Not sure it matters, but still.) Neither
        // object can be `static`.
        let mut rng = rand::thread_rng();
        let power_range: Range<i8> = Range::new(1, 6);
        Player {
            name: PName(String::from(name)),
            team: String::from(team),
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
}

// Does not print the team name or the role.
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}: {}", self.name.0, self.power)
    }
}

#[derive(Debug, Default)]
pub struct PlayersByName(HashMap<PName, Player>);

impl PlayersByName {
    pub fn new() -> PlayersByName {
        PlayersByName(HashMap::new())
    }
    pub fn add(&mut self, p: Player) {
        // Q: Some way to insert using the hash directly instead of cloning the string first, since
        // the actual string isn't really necessary?
        self.0.insert(p.name.clone(), p);
    }
    /* TODO - do I need these?
    pub fn find_ref(&self, name: &PName) -> Option<&Player> {
        self.0.get(name)
    }
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
