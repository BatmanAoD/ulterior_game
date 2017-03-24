use std::collections::HashMap;


#[derive(Debug)]
pub struct Power {
    pub red: Option<u8>,
    pub blue: Option<u8>,
    pub green: Option<u8>
}

// TODO move to separate file
#[derive(Debug)]
pub enum Role {
    Prophet { target: String },
    Traitor,
    // ...etc
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct PName(pub String);

#[derive(Debug)]
// TODO constructor: take NAME only; other attributes are RANODMLY assigned
// (....though the teams should be balanced...)
pub struct Player {
    pub name: PName,
    pub team: String,
    // TODO replace 'public' field with misc accessor functions
    // and the 'fight' functionality (which will mutate by switching from
    // 'some' to 'none' if the player loses)
    pub power: Power,
    // TODO replace 'public' field with accessor functions (roles are NEVER
    // changed; they are ONLY used to (1) impact player behavior and (2)
    // determine victory conditions)
    pub role: Option<Box<Role>>
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
