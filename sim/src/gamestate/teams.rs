use std::collections::HashMap;
use std::fmt;

use crate::gamestate::players::{PName, Player, PlayersByName};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
// TODO Like PName, the intent was to make the inner `String` private and
// only permit creating TNames from this interface.
// For now, just punt on this.
// A better strategy might be to only make some type like `TNameRef(&TName)`
// public, and keep `TName` itself private; this would ensure that outside of
// this module, only references to valid TNames could be acquired.
pub struct TName(pub String);

#[derive(Debug, Default)]
pub struct Team {
    players: PlayersByName,
    honor: i16,
}

impl Team {
    pub fn gain_honor(&mut self, honor: i16) {
        self.honor += honor;
    }

    pub fn pretty_players(&self) -> String {
        Player::pretty_multi(self.players.players())
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  Honor: {}", self.honor)?;
        // TODO: Indent the 'pretty_players' string
        write!(f, "{}", self.pretty_players())
    }
}

#[derive(Debug, Default)]
pub struct TeamsByName(HashMap<TName, Team>);

impl TeamsByName {
    pub fn add(&mut self, t: &str, players: PlayersByName) {
        if self
            .0
            .insert(TName(t.to_owned()), Team { players, honor: 0 })
            .is_some()
        {
            panic!("Team name added twice: {}", t)
        }
    }

    // Panics if the team does not exist
    // (TODO: After revising TName/PName, reconsider this)
    pub fn team_mut(&mut self, t: &TName) -> &mut Team {
        self.0.get_mut(t).expect("Team not found")
    }

    // PName should only be constructed from a *known* player, which is why this method
    // doesn't return an `Option`.
    // TODO: Change this. `PName` doesn't actually have the type-safety
    // that it would need for such a guarantee.
    // Also, this would break if there were multiple game states.
    pub fn player_data(&self, name: &PName) -> &Player {
        for team in self.0.values() {
            if let Some(player) = team.players.find_player(name) {
                return player;
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn player_mut(&mut self, name: &PName) -> &mut Player {
        for team in self.0.values_mut() {
            if let Some(player) = team.players.find_mut(name) {
                return player;
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.0.iter().flat_map(|(_, team)| team.players.players())
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0
            .iter_mut()
            .flat_map(|(_, team)| team.players.players_mut())
    }

    pub fn pretty_player<'a>(&self, name: &'a PName) -> String {
        Player::pretty(self.player_data(name))
    }

    pub fn pretty_players<'a>(&self, names: impl Iterator<Item = &'a PName>) -> String {
        Player::pretty_multi(names.map(|name| self.player_data(name)))
    }
}

impl fmt::Display for TeamsByName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (name, team) in self.0.iter() {
            writeln!(f, "Team {}:", name.0)?;
            writeln!(f, "{}", team)?;
        }
        Ok(())
    }
}
