use std::collections::HashMap;

use crate::gamestate::players;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
// XXX TODO Like PName, the intent was to make the inner `String` private and
// only permit creating TNames from this interface.
// For now, just punt on this.
// A better strategy might be to only make some type like `TNameRef(&TName)`
// public, and keep `TName` itself private; this would ensure that outside of
// this module, only references to valid TNames could be acquired.
pub struct TName(pub String);

#[derive(Debug)]
// XXX TODO teams need to track honor
pub struct TeamsByName(HashMap<TName, players::PlayersByName>);

impl TeamsByName {
    pub fn new() -> TeamsByName {
        TeamsByName(HashMap::new())
    }

    pub fn add(&mut self, t: &str, p: players::PlayersByName) {
        if let Some(_) = self.0.insert(TName(String::from(t)), p) {
            panic!("Team name added twice: {}", t)
        }
    }

    pub fn gain_honor(&mut self, t: &TName, honor: i16) {
        unimplemented!()
    }

    // PName should only be constructed from a *known* player, which is why this method
    // doesn't return an `Option`.
    // XXX TODO: Change this. `PName` doesn't actually have the type-safety
    // that it would need for such a guarantee.
    pub fn find_player(&self, name: &players::PName) -> &players::Player {
        for (_team, players) in &self.0 {
            if let Some(player) = players.find_ref(name) {
                return player
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn find_player_mut(&mut self, name: &players::PName) -> &mut players::Player {
        for (_team, players) in &mut self.0 {
            if let Some(player) = players.find_mut(name) {
                return player
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn players(&self) -> impl Iterator<Item = &players::Player> {
        self.0.iter().flat_map(|(_, players)| players.iter())
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut players::Player> {
        self.0.iter_mut().flat_map(|(_, players)| players.iter_mut())
    }
}
