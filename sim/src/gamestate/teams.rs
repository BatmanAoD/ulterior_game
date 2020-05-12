use std::collections::HashMap;

use crate::gamestate::players;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct TName(String);

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

    pub fn find_player<'a, 'b>(&'a self, name: &'b players::PName) -> &'a players::Player {
        for (_team, players) in &self.0 {
            if let Some(player) = players.find_ref(name) {
                return player
            }
        }
        panic!("Could not find player {:?}", name);
    }

    pub fn find_player_mut<'a, 'b>(&'a mut self, name: &'b players::PName) -> &'a mut players::Player {
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
