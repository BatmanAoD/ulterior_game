use crate::gamestate::players::{PName, Player, PlayersByName};
use crate::gamestate::teams::{TName, TeamsByName};

use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct ActiveGame {
    teams: TeamsByName,
}

impl ActiveGame {
    pub fn new(player_names: impl Iterator<Item=String>, team_names: Vec<String>) -> Self {
        let mut rng = rand::thread_rng();
            let mut player_list = player_names.collect::<Vec<_>>();
            // Randomize player order
            rng.shuffle(&mut player_list);
            let players_per_team = player_list.len() / team_names.len();
            let mut extra_players = player_list.len() % team_names.len();
            let mut team_end: usize;
            let mut teams = TeamsByName::new();
            for (i, team) in team_names.iter().enumerate() {
                let mut players = PlayersByName::new();
                let team_start = i * players_per_team;
                team_end = team_start + players_per_team;
                // Add an extra player to the first (players % teams) teams
                if extra_players > 0 {
                    team_end += 1;
                    extra_players -= 1;
                }
                let players_on_team = player_list.get(team_start..team_end).unwrap();
                for name in players_on_team {
                    players.add(Player::new(name, team));
                }
                teams.add(&team, players);
            }
            ActiveGame{teams}
    }

    // Panics if the team does not exist
    // (TODO: After revising TName/PName, reconsider this)
    pub fn gain_honor(&mut self, t: &TName, honor: i16) {
        self.teams.team_mut(t).gain_honor(honor);
    }

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

impl fmt::Display for ActiveGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Game state:")?;
        writeln!(f, "{}", self.teams)
    }
}