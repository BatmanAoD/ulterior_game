use crate::gamestate;

#[derive(Debug)]
pub struct ActiveGame {
    // Q: Can a struct with non-`pub` fields *only* be initialized within the same module?
    pub teams: gamestate::teams::TeamsByName,
}

impl ActiveGame {
    pub fn get_pname(&self, name: &str) -> Option<&PName> {
        self.players().find(|p| p.name == name)
    }

    pub fn find_player<'a, 'b>(&'a self, player: &'b gamestate::players::PName) -> &'a gamestate::players::Player {
        self.teams.find_player(player)
    }

    pub fn find_player_mut<'a, 'b>(&'a mut self, player: &'b gamestate::players::PName) -> &'a mut gamestate::players::Player {
        self.teams.find_player_mut(player)
    }

    pub fn players(&self) -> impl Iterator<Item = &gamestate::players::Player> {
        self.teams.players()
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut gamestate::players::Player> {
        self.teams.players_mut()
    }
}
