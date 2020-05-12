use std::collections::HashMap;
use std::ops::{Index,IndexMut};

use rand;
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
            _ => panic!("Invalid 'PowerType' values: {}, {}", self as i16, against as i16),
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
    fn randomize(power_range: &Range<i8>, mut rng: &mut rand::ThreadRng) -> Self {
        Power {
            red: ColorPower(Some(power_range.sample(&mut rng))),
            green: ColorPower(Some(power_range.sample(&mut rng))),
            blue: ColorPower(Some(power_range.sample(&mut rng))),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorPower (Option<i8>);

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
// XXX Q: Can `PName` be constructed externally?
pub struct PName(String);

impl PartialEq<&str> for PName {
    fn eq(&self, s: &&str) -> bool {
        self.0 == *s
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: PName,
    pub team: String,   // XXX TODO should be TName
    // Power is only visible with `Player::strength` and modifiable with
    // `Player::lose_power`
    power: Power,
    // Roles are NEVER changed; they are ONLY used to (1) impact player behavior
    // and (2) determine victory conditions
    role: Option<Box<Role>>,
}

impl Player {
    // Q: Better to take String as arg, or &str?
    pub fn new(name: &str, team: &str) -> Player {
        // Q: Can I avoid re-creating these each time? (Not sure it matters, but still.) Neither
        // object can be `static`.
        let mut rng = rand::thread_rng();
        let power_range: Range<i8> = Range::new(1, 6);
        Player {
            // Q: typename-initialization syntax versus non-typename struct initialization seems
            // inconsistent. Why not either `{}` or `()` uniformly?
            // A?: Something about tuples vs structs...? Does that really matter?
            // Q: Why can't initialization just infer that I want all my `&str`s to become
            // `String`s, whenever that's what I'm assigning to?
            name: PName(String::from(name)),
            team: String::from(team),
            // Q: Why can't rustc infer that the braces are initializing a 'Power' struct? I.e., why
            // not just `power: { ....`
            // A?: This inference isn't worthwhile to implement because it would break with
            // function overloading.
            power: Power::randomize(&power_range, &mut rng),
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

// Apparently the Rust standard library currently doesn't provide any hashers other than SipHasher,
// so we can't specify a faster one here.
#[derive(Debug)]
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
    pub fn find_ref(&self, name: &PName) -> Option<&Player> {
        self.0.get(name)
    }
    pub fn find_mut(&mut self, name: &PName) -> Option<&mut Player> {
        self.0.get_mut(name)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Player> {
        self.0.values()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0.values_mut()
    }
}
