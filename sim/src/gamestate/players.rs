use std::collections::HashMap;

use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug)]
struct Power {
    red: Option<i8>,
    blue: Option<i8>,
    green: Option<i8>
}

// TODO move to separate file
#[derive(Debug)]
enum Role {
    Prophet { target: String },
    Traitor,
    // ...etc
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct PName(String);

#[derive(Debug)]
pub struct Player {
    name: PName,
    team: String,
    // TODO replace 'public' field with misc accessor functions
    // and the 'fight' functionality (which will mutate by switching from
    // 'some' to 'none' if the player loses)
    power: Power,
    // TODO replace 'public' field with accessor functions (roles are NEVER
    // changed; they are ONLY used to (1) impact player behavior and (2)
    // determine victory conditions)
    role: Option<Box<Role>>
}

impl Player {
    // TODO randomly assign team
    // (NOTE: Need to CHANGE AllPlayers to require a *complete* list of players all at once. To
    // facilitate this, it would probably be best to have a "GameBuilder" class somewhere that
    // would validate input, construct the full player list, permit setting team names, etc.)
    pub fn new(name: String , team: String ) -> Player {
        // Q: Can I avoid re-creating these each time? (Not sure it matters, but still.) Neither
        // object can be `static`.
        let mut rng = rand::thread_rng();
        let power_range: Range<i8> = Range::new(1,6);
        Player {
            // Q: typname-initialization syntax versus non-typename struct initialization seems
            // inconsistent. Why not either `{}` or `()` uniformly?
            name: PName( name ),
            team: team,
            // Q: Why can't rustc infer that the braces are initializing a 'Power' struct? I.e., why
            // not just `power: { ....`
            // A?: This inference isn't worthwhile to implement because it would break with
            // function overloading.
            power: Power {
                red: Some(power_range.ind_sample(&mut rng)),
                blue: Some(power_range.ind_sample(&mut rng)),
                green: Some(power_range.ind_sample(&mut rng))
            },
            role: None
        }
    }
}

// Apparently the Rust standard library currently doesn't provide any hashers other than SipHasher,
// so we can't specify a faster one here.
#[derive(Debug)]
pub struct AllPlayers(HashMap<PName, Player>);

impl AllPlayers {
    pub fn new() -> AllPlayers {
        AllPlayers(HashMap::new())
    }
    pub fn add(&mut self, p: Player ) {
        self.0.insert(p.name.clone(), p);
    }
}
