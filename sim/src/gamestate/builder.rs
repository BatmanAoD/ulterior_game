use std::collections::BTreeSet;

use quick_error::quick_error;
use rand::Rng;

use crate::gamestate;

#[derive(Debug)]
pub struct Setup {
    team_set: TeamSet,
    player_names: BTreeSet<String>,
}

#[derive(Clone, Debug)]
enum TeamSet {
    Partial(BTreeSet<String>),
    Complete(Vec<String>),
}

// Q: Possible to somehow just infer that `TeamSet` should provide a `len` method since all
// variants do?
impl TeamSet {
    pub fn len(&self) -> usize {
        match *self {
            // Q: Why is it necessary to specify the enum name when matching on an enum?
            TeamSet::Partial(ref p) => p.len(),
            TeamSet::Complete(ref c) => c.len(),
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum AddTeamErr {
        PlayersAlreadyAdded {}
        TeamAlreadyExists {}
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum AddPlayerErr {
        TeamsNotEstablished {}
        PlayerNameDuplicated {}
    }

}

quick_error! {
    #[derive(Debug)]
    pub enum StartGameErr {
        TeamsNotEstablished {}
        TooFewPlayers {}
    }
}

// Q: Why doesn't `must_use` actually trigger a warning when the return value is ignored in `main`?
// #[must_use]
// pub type OptErr<E> = Option<E>;

// Q: Does something like this already exist?
pub type OptErr<E> = Result<(), E>;

impl Setup {
    pub fn new_game() -> Setup {
        Setup {
            team_set: TeamSet::Partial(BTreeSet::new()),
            player_names: BTreeSet::new(),
        }
    }

    pub fn finalize(self) -> Result<gamestate::active::ActiveGame, StartGameErr> {
        // TODO DESIGN min number of players?
        if self.player_names.len() < self.team_set.len() * 3 {
            return Err(StartGameErr::TooFewPlayers);
        }

        let mut rng = rand::thread_rng();

        match self.team_set {
            // Q: See note below about scopes for variants
            TeamSet::Partial(_) => Err(StartGameErr::TeamsNotEstablished),
            TeamSet::Complete(ref team_names) => {
                // TODO most of this logic should be moved into one or more functions.
                // Perhaps an `ActiveGame` constructor?

                // Randomize player names
                let mut player_list = self.player_names.iter().cloned().collect::<Vec<_>>();
                rng.shuffle(&mut player_list);
                let players_per_team = self.player_names.len() / team_names.len();
                let mut extra_players = self.player_names.len() % team_names.len();
                let mut team_end: usize;
                let mut teams = gamestate::teams::TeamsByName::new();
                for (i, team) in team_names.iter().enumerate() {
                    let mut players = gamestate::players::PlayersByName::new();
                    let team_start = i * players_per_team;
                    team_end = team_start + players_per_team;
                    // Add an extra player to the first (players % teams) teams
                    if extra_players > 0 {
                        team_end += 1;
                        extra_players -= 1;
                    }
                    let players_on_team = player_list.get(team_start..team_end).unwrap();
                    for name in players_on_team {
                        players.add(gamestate::players::Player::new(name, team));
                    }
                    teams.add(&team, players);
                }
                Ok(gamestate::active::ActiveGame { teams })
            }
        }
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

    pub fn add_team(&mut self, name: &str) -> OptErr<AddTeamErr> {
        match self.team_set {
            // Q: Why is `Err` in scope without a `use`?
            TeamSet::Complete(_) => Err(AddTeamErr::PlayersAlreadyAdded),
            TeamSet::Partial(ref mut set) => {
                let already_exists = !set.insert(String::from(name));
                if already_exists {
                    Err(AddTeamErr::TeamAlreadyExists)
                }
                else {
                    Ok(())
                }
            }
        }
    }

    pub fn add_player(&mut self, name: &str) -> OptErr<AddPlayerErr> {
        // Q: Same as above about scope of `Err` vs my error enum
        if self.team_set.len() < 2 {
            return Err(AddPlayerErr::TeamsNotEstablished);
        }

        let final_team = match self.team_set {
            TeamSet::Partial(ref set) => TeamSet::Complete(set.iter().cloned().collect()),
            // Q: When `TeamSet` doesn't implement `Clone`, this gives a confusing error:
            // 'match arms have incompatible types...found &TeamSet'
            // ....can this be made nicer? It's not clear why the compiler can't tell that
            // `.clone()` simply *can't be called* on `complete`, rather than guessing that
            // `complete.clone()` is a *reference*.
            ref complete => complete.clone(),
        };
        self.team_set = final_team;

        // Alternate versions of the above:
        // This works as well, but seems slightly more awkward than the above.
        //        let mut complete = self.team_set.clone();
        //        if let TeamSet::Partial(ref partial) = self.team_set {
        //            complete = TeamSet::Complete(partial.into_iter().cloned().collect());
        //        }
        //        self.team_set = complete;

        // This works but is awkward. Two match statements! Explicit variable typing!
        // ...note that `finalized` could be a `mut None`, which would infer `<Vec<String>>`
        //        let finalized: Option<Vec<String>>;
        //        match self.team_set {
        //            TeamSet::Partial(ref set) => {
        //                finalized = Some(set.into_iter().cloned().collect());
        //            }
        //            _ => finalized = None,
        //        }
        //
        //        match finalized {
        //            Some(teams) => self.team_set = TeamSet::Complete(teams),
        //            None => {}
        //        }

        let already_exists = !self.player_names.insert(String::from(name));
        if already_exists {
            return Err(AddPlayerErr::PlayerNameDuplicated);
        }
        Ok(())
    }
}
