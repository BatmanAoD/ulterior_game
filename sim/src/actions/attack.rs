use rand::Rng;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
// use crate::gamestate::teams;

#[derive(Debug)]
pub struct Attack {
    attackers: Vec<String>,
    defenders: Vec<String>,
    def_power: PowerType,
    att_power: PowerType,
}

impl Attack {
    pub fn apply(self, state: &mut ActiveGame) {
        unimplemented!();
    }
}

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

    pub fn finalize(self, att_power: PowerType) -> Attack {
        Attack {
            attackers: self.attackers,
            defenders: self.defenders,
            def_power: self.def_power,
            att_power,
        }
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
        let mut rng = rand::thread_rng();
        AddAttacker {
            attack: self.attack,
            att_power: rng.gen(),
        }
    }
}

pub struct AddAttacker {
    attack: DeclaredAttack,
    att_power: PowerType,
}

impl AddAttacker {
    pub fn add(mut self, name: &str) -> Self {
        // TODO warn if attacker is on defender's team?
        self.attack.attackers.push(name.to_owned());
        self
    }
    pub fn finalize_offense(self) -> Attack {
        self.attack.finalize(self.att_power)
    }
}
