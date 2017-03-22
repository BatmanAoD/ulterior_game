#[derive(Debug)]
pub struct Power {
    pub red: Option<u8>,
    pub blue: Option<u8>,
    pub green: Option<u8>
}

// TODO move to separate file
#[derive(Debug)]
pub enum Role {
    Prophet,    // TODO ref to player...? How?
    Traitor,
    // ...etc
}

#[derive(Debug)]
// TODO constructor: take NAME only; other attributes are RANODMLY assigned
// (....though the teams should be balanced...)
pub struct Player {
    pub name: String,
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


