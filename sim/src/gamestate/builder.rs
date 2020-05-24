use std::collections::BTreeSet;

use quick_error::quick_error;

use crate::gamestate::active::ActiveGame;

#[derive(Debug)]
pub struct Setup {
    team_names: BTreeSet<String>,
    player_names: BTreeSet<String>,
}

// Q: Possible to somehow derive methods for enums when all variants provide
// that method?
// Q: Why is it necessary to specify the enum name when matching on an enum?
// (Except `Result`, it seems?)

quick_error! {
    #[derive(Debug)]
    pub enum GameSetupErr {
        PlayerNameDuplicated {}
        TeamNameDuplicated {}
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum StartGameErr {
        TeamsNotEstablished {}
        TooFewPlayers {}
    }
}

pub type OptErr<E> = Result<(), E>;

impl Setup {
    pub fn new_game() -> Setup {
        Setup {
            team_names: BTreeSet::new(),
            player_names: BTreeSet::new(),
        }
    }

    pub fn finalize(self) -> Result<ActiveGame, StartGameErr> {
        if self.team_names.len() < 2 {
            return Err(StartGameErr::TeamsNotEstablished);
        }
        // TODO DESIGN min number of players?
        if self.player_names.len() < self.team_names.len() * 3 {
            return Err(StartGameErr::TooFewPlayers);
        }

        Ok(ActiveGame::new(
            self.player_names.into_iter(),
            self.team_names.into_iter(),
        ))
    }

    pub fn add_team_or_panic(mut self, name: &str) -> Self {
        self.add_team(name).unwrap();
        println!("Added team: {}", name);
        self
    }

    pub fn add_player_or_panic(mut self, name: &str) -> Self {
        self.add_player(name).unwrap();
        println!("Added player: {}", name);
        self
    }

    pub fn add_team(&mut self, name: &str) -> OptErr<GameSetupErr> {
        let already_exists = !self.team_names.insert(String::from(name));
        if already_exists {
            Err(GameSetupErr::TeamNameDuplicated)
        } else {
            Ok(())
        }
    }

    pub fn add_player(&mut self, name: &str) -> OptErr<GameSetupErr> {
        let already_exists = !self.player_names.insert(String::from(name));
        if already_exists {
            Err(GameSetupErr::PlayerNameDuplicated)
        } else {
            Ok(())
        }
    }
}
