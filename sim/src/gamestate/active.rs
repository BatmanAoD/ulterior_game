use crate::gamestate::teams::{TeamsByName, TName};
use crate::gamestate::players::{Player, PName};

#[derive(Debug)]
pub struct ActiveGame {
    pub teams: TeamsByName,
}

impl ActiveGame {
    pub fn player_by_name(&self, name: &str) -> Option<(PName, TName)> {
        self.players().find(|p| p.name == name).map(|p| (p.name.to_owned(), TName(p.team.to_owned())))
    }

    pub fn find_player<'a, 'b>(&'a self, player: &'b PName) -> &'a Player {
        self.teams.find_player(player)
    }

    pub fn find_player_mut<'a, 'b>(&'a mut self, player: &'b PName) -> &'a mut Player {
        self.teams.find_player_mut(player)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.teams.players()
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.teams.players_mut()
    }
}
