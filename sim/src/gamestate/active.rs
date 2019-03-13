use crate::gamestate;

#[derive(Debug)]
pub struct ActiveGame {
    // Q: Can a struct with non-`pub` fields *only* be initialized within the same module?
    pub teams: gamestate::teams::TeamsByName,
}

impl ActiveGame {
    pub fn find_player<'a, 'b>(&'a self, player: &'b str) -> &'a gamestate::players::Player {
        self.teams.find_player(player)
    }

    pub fn find_player_mut<'a, 'b>(&'a mut self, player: &'b str) -> &'a mut gamestate::players::Player {
        self.teams.find_player_mut(player)
    }
}
