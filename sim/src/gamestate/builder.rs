use std::collections::BTreeSet;

// Q: Isn't there some way to specify "here in the same parent module" instead of calling it
// (gamestate) explicitly by name?
use gamestate::players;

pub struct Setup {
    team_set: TeamSet,
    player_names: BTreeSet<String>
}

enum TeamSet {
    partial (BTreeSet<String>),
    complete (Vec<String>)
}

// Q: Possible to somehow just infer that `TeamSet` should provide a `len` method since all
// variants do?
impl TeamSet {
    pub fn len(&self) -> usize {
        return match *self {
            // Q: Why is it necessary to specify the enum name when matching on an enum?
            TeamSet::partial(ref p) => p.len(),
            TeamSet::complete(ref c) => c.len()
        }
    }
}

#[derive(Debug)]
pub enum AddTeamErr {
    PlayersAlreadyAdded,
    TeamAlreadyExists
}

#[derive(Debug)]
pub enum AddPlayerErr {
    TeamsNotEstablished,
    PlayerNameDuplicated
}

// Q: Does something like this already exist?
#[must_use]
pub type OptErr<E> = Option<E>;

impl Setup {
    pub fn new_game() -> Setup {
        Setup {
            team_set: TeamSet::partial(BTreeSet::new()),
            player_names: BTreeSet::new()
        }
    }

    // pub fn finalize() -> gamestate::ActiveGame {
    // Active game should have an `AllPlayers` method and somehow represent teams, etc
    // For player in the list of player names: {
        // let mut rng = rand::thread_rng();
        // let power_range: Range<i8> = Range::new(0, team_set.len() - 1);
        //
        // // TODO easiest way to get length when we *know* the enum is `complete`?
        // let p = gamestate::players::Player::new(
        //     name, team_set.complete[power_range.ind_sample(&mut rng)]);
        // }
    // }

    pub fn add_team(&mut self, name: String) -> OptErr<AddTeamErr> {
        match self.team_set {
            // Q: Why is `Some` in scope (without specifying `Option` or `OptErr`, but `AddTeamErr`
            // members are not?
            TeamSet::complete(_) => return Some(AddTeamErr::PlayersAlreadyAdded),
            TeamSet::partial(ref mut set) => {
                let already_exists = set.insert(name);
                if already_exists { return Some(AddTeamErr::TeamAlreadyExists) }
                None
            }
        }
    }

    pub fn add_player(&mut self, name: String) -> OptErr<AddPlayerErr> {
        // Q: Same as above about scope of `Some` vs my error enum
        if self.team_set.len() < 2 { return Some(AddPlayerErr::TeamsNotEstablished) }
        // Q: Way to infer/deduce type of `teams` here?
        // let teams: &TeamSet;
        // Q: How on *earth* do I conditionally re-assign an enum to a different variant, based on
        // the variant it is when we enter the function?
        // This `Option` thing seems like a terrible kludge, and it's really annoying.
        // Q: Can I infer the type based on the variant I want?
        let finalized: Option<Vec<String>>;
        match self.team_set {
            TeamSet::partial(ref set) => {
                finalized = Some(set.into_iter().cloned().collect());
            }
            _ => finalized = None,
        }

        match finalized {
            Some(teams) => self.team_set = TeamSet::complete(teams),
            None => {}
        }

        let already_exists = self.player_names.insert(name);
        if already_exists { return Some(AddPlayerErr::PlayerNameDuplicated) }
        None
    }
}
