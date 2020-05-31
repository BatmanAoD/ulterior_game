use std::collections::BTreeSet;
use std::fmt;

use itertools::Itertools;
use quick_error::quick_error;
use rand::distributions::Uniform;
use rand::seq::IteratorRandom;
use rand::Rng;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::{PlayerAttributePool, Role};
use crate::gamestate::power::Power;

#[derive(Default)]
pub struct Setup {
    team_names: BTreeSet<String>,
    player_names: BTreeSet<String>,
}

struct PlayerAttributeProvider {
    power_token_sets: Vec<Power>,
    num_players_remaining: usize,
    roles: Vec<Role>,
    destined: BTreeSet<String>,
}

impl PlayerAttributeProvider {
    fn new(player_names: &BTreeSet<String>) -> Self {
        let num_players = player_names.len();
        let mut rng = rand::thread_rng();
        let power_range: Uniform<i8> = Uniform::new(1, 6);
        let mut pool = PlayerAttributeProvider {
            power_token_sets: Vec::with_capacity(num_players),
            num_players_remaining: num_players,
            roles: Default::default(),
            destined: Default::default(),
        };
        pool.power_token_sets.resize_with(
            num_players,
            // TODO: These should not be randomized independently
            || Power::randomize(power_range, &mut rng),
        );
        // Q: How many 'destined'?
        let destined = player_names
            .iter()
            .choose(&mut rng)
            .expect("No players in game")
            .clone();
        pool.destined.insert(destined.clone());
        pool.roles.push(Role::Prophet { target: destined });
        pool.roles.push(Role::Traitor);
        pool
    }
}

impl PlayerAttributePool for PlayerAttributeProvider {
    fn next_power(&mut self) -> Power {
        self.power_token_sets
            .pop()
            .expect("No more power tokens left")
    }
    fn next_role(&mut self, name: &str) -> Option<Role> {
        self.num_players_remaining -= 1;
        if self.destined.contains(name) {
            return Some(Role::Destined);
        }
        let probability_has_role =
            self.roles.len() as f64 / (self.num_players_remaining as f64 + 1.0);
        let mut rng = rand::thread_rng();
        if rng.gen_bool(probability_has_role) {
            let index = rng.sample(Uniform::new(0, self.roles.len()));
            Some(self.roles.remove(index))
        } else {
            None
        }
    }
    fn is_empty(&self) -> bool {
        self.power_token_sets.is_empty() && self.num_players_remaining == 0 && self.roles.is_empty()
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
        let num_players = self.player_names.len();
        if num_players < self.team_names.len() * 3 {
            return Err(StartGameErr::TooFewPlayers);
        }

        let attributes_provider = PlayerAttributeProvider::new(&self.player_names);
        Ok(ActiveGame::new(
            self.player_names.into_iter(),
            self.team_names.into_iter(),
            attributes_provider,
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
        write!(
            f,
            "Teams: {}, Players: {}",
            self.team_names.iter().join(", "),
            self.player_names.iter().join(", ")
        )
    }
}
