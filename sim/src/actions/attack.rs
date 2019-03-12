use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
use crate::gamestate::teams;

// XXX TODO this must contain (and calculate in its constructor) the effect on the gamestate.
// This includes:
//  * Losing players' token loss
//  * Honor gained by winning players' team
pub struct Outcome {}

#[derive(Debug)]
pub struct DeclaredAttack {
    attackers: Vec<String>,
    defenders: Vec<String>,
    def_power: PowerType,
}

impl DeclaredAttack {
    // Initiates an attack, returning a closure over the data necessary to perform the next step of the
    // attack.
    pub fn declare(attacker: &str, defender: &str, def_power: PowerType) -> AddDefender {
        let attack = DeclaredAttack {
            attackers: vec![String::from(attacker)],
            defenders: vec![String::from(defender)],
            def_power: def_power,
        };

        AddDefender { attack }
    }

    pub fn finalize(self) -> Outcome {
        println!("Attack state when finalized: {:#?}", self);
        unimplemented!();
    }
}

pub struct AddDefender {
    attack: DeclaredAttack,
}

impl AddDefender {
    pub fn add(mut self, name: &str) -> Self {
        // TODO warn if defender is on attacker's team?
        self.attack.defenders.push(name.to_owned());
        self
    }
    pub fn finalize_defense(self) -> AddAttacker {
        AddAttacker {
            attack: self.attack,
        }
    }
}

pub struct AddAttacker {
    attack: DeclaredAttack,
}

impl AddAttacker {
    pub fn add(mut self, name: &str) -> Self {
        // TODO warn if attacker is on defender's team?
        self.attack.attackers.push(name.to_owned());
        self
    }
    pub fn finalize_offense(self) -> Outcome {
        self.attack.finalize()
    }
}
