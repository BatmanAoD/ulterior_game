use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
use crate::gamestate::teams;

// XXX TODO this must contain (and calculate in its constructor) the effect on the gamestate.
// This includes:
//  * Losing players' token loss
//  * Honor gained by winning players' team
pub struct Outcome {}

struct DeclaredAttack {
    attackers: Vec<String>,
    defenders: Vec<String>,
    def_power: PowerType,
}

impl DeclaredAttack {
    // Initiates an attack, returning a closure over the data necessary to perform the next step of the
    // attack.
    fn declare(attacker: &str, defender: &str, def_power: PowerType) -> AddDefender {
        let attack = DeclaredAttack {
            attackers: vec![String::from(attacker)],
            defenders: vec![String::from(defender)],
            def_power: def_power,
        };

        // Q: Any way to infer the inner `AddDefender` type?
        AddDefender { attack }
    }

    fn finalize(self) -> Outcome {
        unimplemented!();
    }
}

struct AddDefender {
    attack: DeclaredAttack,
}

impl AddDefender {
    fn add(&mut self, name: &str) {
        self.attack.defenders.push(name.to_owned());
    }
    fn finalize_defense(self) -> AddAttacker {
        AddAttacker {
            attack: self.attack,
        }
    }
}

struct AddAttacker {
    attack: DeclaredAttack,
}

impl AddAttacker {
    fn add(&mut self, name: &str) {
        self.attack.attackers.push(name.to_owned());
    }
    fn finalize_offense(self) -> Outcome {
        self.attack.finalize()
    }
}
