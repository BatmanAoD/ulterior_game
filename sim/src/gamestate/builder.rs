use std::collections::BTreeSet;

use rand;
use rand::distributions::Range;
use rand::Rng;

// Q: Isn't there some way to specify "here in the same parent module" instead of calling it
// (gamestate) explicitly by name?
use gamestate;

#[derive(Debug)]
pub struct Setup {
    team_set: TeamSet,
    player_names: BTreeSet<String>,
}

#[derive(Clone,Debug)]
enum TeamSet {
    Partial(BTreeSet<String>),
    Complete(Vec<String>),
}

// Q: Possible to somehow just infer that `TeamSet` should provide a `len` method since all
// variants do?
impl TeamSet {
    pub fn len(&self) -> usize {
        return match *self {
                   // Q: Why is it necessary to specify the enum name when matching on an enum?
                   TeamSet::Partial(ref p) => p.len(),
                   TeamSet::Complete(ref c) => c.len(),
               };
    }
}

#[derive(Debug)]
pub enum AddTeamErr {
    PlayersAlreadyAdded,
    TeamAlreadyExists,
}

#[derive(Debug)]
pub enum AddPlayerErr {
    TeamsNotEstablished,
    PlayerNameDuplicated,
}

#[derive(Debug)]
pub enum StartGameErr {
    TeamsNotEstablished,
    TooFewPlayers,
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

    // Q: Does this actually 'move' `self` so that the struct can't be re-used after calling? If
    // so, great.
    pub fn finalize(self) -> Result<gamestate::active::ActiveGame, StartGameErr> {
        // XXX TODO min number of players?
        if self.player_names.len() < self.team_set.len() * 3 {
            return Err(StartGameErr::TooFewPlayers);
        }

        let mut rng = rand::thread_rng();

        match self.team_set {
            // Q: See note below about scopes for variants
            TeamSet::Partial(_) => return Err(StartGameErr::TeamsNotEstablished),
            TeamSet::Complete(ref team_names) => {
                // Q: With the match, this begins to seem like some deep "pyramid" indenting...ways
                // to avoid?

                // Randomize player names
                let mut player_list = self.player_names.iter().cloned().collect::<Vec<_>>();
                rng.shuffle(&mut player_list);
                let players_per_team = self.player_names.len() / team_names.len();
                let mut extra_players = self.player_names.len() % team_names.len();
                let mut team_end: usize;
                let mut teams = gamestate::teams::TeamsByName::new();
                for (i, team) in team_names.into_iter().enumerate() {
                    let mut players = gamestate::players::PlayersByName::new();
                    let team_start = i * players_per_team;
                    team_end = team_start + players_per_team;
                    // Add an extra player to the first (players % teams) teams
                    if extra_players > 0 {
                        team_end += 1;
                        extra_players -= 1;
                    }
                    let players_on_team = player_list.get(team_start..team_end).unwrap();
                    for ref name in players_on_team {
                        players.add(gamestate::players::Player::new(&name, team));
                    }
                    teams.add(&team, players);
                }
                Ok(gamestate::active::ActiveGame { teams: teams })
            }
        }
    }

    pub fn add_team_or_panic(&mut self, name: &str) {
        // XXX TODO print the *name*, not '()', on success
        println!("{:?}", self.add_team(name).unwrap());
    }

    pub fn add_player_or_panic(&mut self, name: &str) {
        // XXX TODO print the *name*, not '()', on success
        println!("{:?}", self.add_player(name).unwrap());
    }

    pub fn add_team(&mut self, name: &str) -> OptErr<AddTeamErr> {
        match self.team_set {
            // Q: Why is `Err` in scope (without specifying `Option` or `OptErr`, but `AddTeamErr`
            // members are not?
            TeamSet::Complete(_) => return Err(AddTeamErr::PlayersAlreadyAdded),
            TeamSet::Partial(ref mut set) => {
                let already_exists = !set.insert(String::from(name));
                if already_exists {
                    return Err(AddTeamErr::TeamAlreadyExists);
                }
                // Q: seriously?
                Ok(())
            }
        }
    }

    pub fn add_player(&mut self, name: &str) -> OptErr<AddPlayerErr> {
        // Q: Same as above about scope of `Err` vs my error enum
        if self.team_set.len() < 2 {
            return Err(AddPlayerErr::TeamsNotEstablished);
        }

        let final_team = match self.team_set {
            TeamSet::Partial(ref set) => {
                TeamSet::Complete(set.into_iter().cloned().collect())
            },
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
