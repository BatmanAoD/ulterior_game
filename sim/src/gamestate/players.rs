use std::collections::HashMap;
use std::fmt;

use crate::gamestate::power::{Power, PowerType};

// TODO move to separate file
#[derive(Clone)]
pub enum Role {
    Prophet { target: String },
    Traitor,
    Destined,
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

pub trait PlayerAttributePool {
    fn next_power(&mut self) -> Power;
    // Roles are not independent, so the name is required
    fn next_role(&mut self, name: &str) -> Option<Role>;

    // Only for assertions or bookkeeping
    fn is_empty(&self) -> bool;
}

#[derive(Clone)]
pub struct Player {
    pub name: PName,
    pub team: String, // TODO should be TName
    // Power is only visible with `Player::strength` and modifiable with
    // `Player::lose_power`
    power: Power,
    // Roles are NEVER changed; they are ONLY used to (1) impact player behavior
    // and (2) determine victory conditions
    role: Option<Role>,
}

impl Player {
    pub fn new(name: String, team: &str, attribute_pool: &mut dyn PlayerAttributePool) -> Player {
        let role = attribute_pool.next_role(&name);
        Player {
            name: PName(name),
            team: team.to_owned(),
            power: attribute_pool.next_power(),
            role,
        }
    }

    pub fn strength(&self, ptype: PowerType) -> i8 {
        self.power[ptype].into()
    }

    pub fn has_power(&self, ptype: PowerType) -> bool {
        self.power[ptype].nonempty()
    }

    pub fn lose_power(&mut self, ptype: PowerType) {
        self.power[ptype].discard()
    }

    pub fn pretty(player: &'_ Player) -> String {
        format!("{}", player)
    }

    pub fn pretty_multi<'a>(players: impl Iterator<Item = &'a Player>) -> String {
        let mut formatted = String::new();
        for player in players {
            // Newlines are added by the `Player` formatter.
            // TODO use an actual columnar formatter.
            formatted = format!("{}{}", &formatted, player);
        }
        formatted
    }

    pub fn format_role(&self) -> String {
        match &self.role {
            Some(Role::Prophet{ target: t }) => format!("Prophet: protect {} at all costs", t),
            Some(Role::Traitor) => "Traitor: try not to let the destined one survive".to_owned(),
            // Destined players do not know that they are Destined.
            _ => "Few know their own destiny; you have yet to discover yours.".to_owned(),
        }
    }
}

// Does not included the team name (which can be printed separately) or the role
// (which is secret and should not be printed)
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:>12}: {}", self.name.0, self.power)
    }
}

#[derive(Clone, Default)]
pub struct PlayersByName(HashMap<PName, Player>);

impl PlayersByName {
    pub fn from(
        team: &str,
        names: impl Iterator<Item = String>,
        attribute_pool: &mut dyn PlayerAttributePool,
    ) -> Self {
        let mut map = HashMap::new();
        for name in names {
            map.insert(PName(name.clone()), Player::new(name, team, attribute_pool));
        }
        PlayersByName(map)
    }
    pub fn find_player(&self, name: &PName) -> Option<&Player> {
        self.0.get(name)
    }
    pub fn find_mut(&mut self, name: &PName) -> Option<&mut Player> {
        self.0.get_mut(name)
    }
    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.0.values()
    }
    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0.values_mut()
    }
}
