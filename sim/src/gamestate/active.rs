use crate::gamestate::players::{PName, Player};
use crate::gamestate::teams::{TName, TeamsByName};

#[derive(Debug)]
pub struct ActiveGame {
    pub teams: TeamsByName,
}

impl ActiveGame {
    pub fn player_by_name(&self, name: &str) -> Option<(PName, TName)> {
        self.players()
            .find(|p| p.name == name)
            .map(|p| (p.name.to_owned(), TName(p.team.to_owned())))
    }

    /*  TODO - do I need these?
    pub fn find_player(&self, player: &PName) -> &Player {
        self.teams.find_player(player)
    }

    pub fn find_player_mut(&mut self, player: &PName) -> &mut Player {
        self.teams.find_player_mut(player)
    }
    */

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.teams.players()
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.teams.players_mut()
    }
}
