use std::collections::BTreeSet;
use std::fmt;

use itertools::Itertools;
use rand::distributions::Range;
use quick_error::quick_error;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::{PlayerAttributePool, Role};
use crate::gamestate::power::Power;

#[derive(Default)]
pub struct Setup {
    team_names: BTreeSet<String>,
    player_names: BTreeSet<String>,
    attribute_pool: PlayerAttributeProvider,
}

struct PlayerAttributeProvider {
    power_token_sets: Vec<Power>,
    roles: Vec<Option<Role>>,
}

impl Default for PlayerAttributeProvider {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let power_range: Range<i8> = Range::new(1, 6);
        unimplemented!()
    }
}

impl PlayerAttributePool for PlayerAttributeProvider {
    fn next_power(&mut self) -> Power {
        unimplemented!()
    }
    fn next_role(&mut self) -> Option<Role> {
        unimplemented!()
    }
    fn is_empty(&mut self) -> bool {
        unimplemented!()
    }
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
        Default::default()
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
            self.attribute_pool,
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

impl fmt::Display for Setup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Teams: {}, Players: {}",
            self.team_names.iter().join(", "),
            self.player_names.iter().join(", ")
        )
    }
}