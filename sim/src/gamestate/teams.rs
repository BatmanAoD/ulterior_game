use std::collections::HashMap;

use crate::gamestate::players::{PName, Player, PlayersByName};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
// TODO Like PName, the intent was to make the inner `String` private and
// only permit creating TNames from this interface.
// For now, just punt on this.
// A better strategy might be to only make some type like `TNameRef(&TName)`
// public, and keep `TName` itself private; this would ensure that outside of
// this module, only references to valid TNames could be acquired.
pub struct TName(pub String);

#[derive(Debug)]
pub struct Team {
    players: PlayersByName,
    honor: i16,
}

#[derive(Debug)]
pub struct TeamsByName(HashMap<TName, Team>);

impl TeamsByName {
    pub fn new() -> TeamsByName {
        TeamsByName(HashMap::new())
    }

    pub fn add(&mut self, t: &str, players: PlayersByName) {
        if let Some(_) = self
            .0
            .insert(TName(String::from(t)), Team { players, honor: 0 })
        {
            panic!("Team name added twice: {}", t)
        }
    }

    // Panics if the team does not exist
    // (TODO: After revising TName/PName, reconsider this)
    pub fn gain_honor(&mut self, t: &TName, honor: i16) {
        self.0.get_mut(t).expect("Team not found").honor += honor
    }

    // PName should only be constructed from a *known* player, which is why this method
    // doesn't return an `Option`.
    // TODO: Change this. `PName` doesn't actually have the type-safety
    // that it would need for such a guarantee.
    pub fn find_player(&self, name: &PName) -> &Player {
        for (_team, team) in &self.0 {
            if let Some(player) = team.players.find_ref(name) {
                return player;
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn find_player_mut(&mut self, name: &PName) -> &mut Player {
        for (_team, team) in &mut self.0 {
            if let Some(player) = team.players.find_mut(name) {
                return player;
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.0.iter().flat_map(|(_, team)| team.players.iter())
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0
            .iter_mut()
            .flat_map(|(_, team)| team.players.iter_mut())
    }
}
