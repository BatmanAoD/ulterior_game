use std::collections::HashMap;

use crate::gamestate::players;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct TName(String);

#[derive(Debug)]
pub struct TeamsByName(HashMap<TName, players::PlayersByName>);

impl TeamsByName {
    pub fn new() -> TeamsByName {
        TeamsByName(HashMap::new())
    }

    pub fn add(&mut self, t: &str, p: players::PlayersByName) {
        self.0.insert(TName(String::from(t)), p);
    }

    pub fn find_player<'a, 'b>(&'a self, player: &'b str) -> &'a players::Player {
        unimplemented!();
    }

    pub fn find_player_mut<'a, 'b>(&'a mut self, player: &'b str) -> &'a mut players::Player {
        unimplemented!();
    }
}
