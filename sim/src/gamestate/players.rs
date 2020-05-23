use std::collections::HashMap;
use std::fmt;

use rand::distributions::Range;
use rand_derive::Rand;

use crate::gamestate::power::{Power, PowerType, ColorPower};

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
