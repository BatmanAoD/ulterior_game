use crate::gamestate;

#[derive(Debug)]
pub struct ActiveGame {
    // Q: Can a struct with non-`pub` fields *only* be initialized within the same module?
    pub teams: gamestate::teams::TeamsByName,
}
