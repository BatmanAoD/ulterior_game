use crate::actions::attack::AttackOutcome;
use crate::gamestate::teams::TeamsByName;

#[derive(Debug)]
pub struct GameHistory {
    initial_state: TeamsByName,
    states: Vec<GameStateNode>,
}

#[derive(Debug)]
struct GameStateNode(AttackOutcome);

impl GameHistory {
    pub fn apply_attack(&mut self, attack: AttackOutcome) {
        self.states.push(GameStateNode(attack));
    }
}
