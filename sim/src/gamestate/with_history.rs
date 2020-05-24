use crate::actions::attack::AttackOutcome;
use crate::gamestate::teams::TeamsByName;

#[derive(Debug)]
pub struct GameHistory {
    initial_state: TeamsByName,
    states: Vec<GameStateNode>,
    current: Option<usize>,
}

#[derive(Debug)]
struct GameStateNode{
    data: AttackOutcome,
    previous: Option<usize>,
    // next: Option<usize>, // ignore for now
}

impl GameHistory {
    pub fn starting_with(initial_state: TeamsByName) -> GameHistory {
        GameHistory {
            initial_state,
            states: Vec::new(),
            current: None,
        }
    }

    pub fn current_state(&self) -> &TeamsByName {
        match self.current {
            Some(index) => &self.states[index].data.new_state,
            None => &self.initial_state,
        }
    }

    pub fn current_mut(&mut self) -> &mut TeamsByName {
        match self.current {
            Some(index) => &mut self.states[index].data.new_state,
            None => &mut self.initial_state,
        }
    }

    pub fn apply_attack(&mut self, attack: AttackOutcome) {
        self.states.push(GameStateNode{data: attack, previous: self.current});
        self.current = Some(self.states.len() - 1)
    }
}
