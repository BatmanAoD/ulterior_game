use std::collections::HashMap;

use rand;
use rand::Rng;
use rand::distributions::{Sample, Range};

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
// TODO constructor: take NAME only; other attributes are RANODMLY assigned
// (....though the teams should be balanced...)
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
    // (Note: it seems that teams can *only* be balanced if the total number of players is known in
    // advance, or if assignment is not actually random.)
    pub fn new(name: String , team: String ) -> Player {
        // Q: Can I avoid re-creating these each time? (Not sure it matters, but still.) Neither
        // object can be `static`.
        let mut rng = rand::weak_rng();
        let mut PowerRange: Range<i8> = Range::new(1,6);
        Player {
            // Q: typname-initialization syntax versus non-typename struct initialization seems
            // inconsistent. Why not either `{}` or `()` uniformly?
            name: PName( name ),
            team: team,
            // Q: Why can't rustc infer that the braces are initializing a 'Power' struct? I.e., why
            // not just `power: { ....`
            power: Power {
                red: Some(PowerRange.sample(&mut rng)),
                blue: Some(PowerRange.sample(&mut rng)),
                green: Some(PowerRange.sample(&mut rng))
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
