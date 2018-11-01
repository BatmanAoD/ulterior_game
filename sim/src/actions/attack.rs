use crate::gamestate::players::PowerType;
use crate::gamestate::teams;
use crate::gamestate::active::ActiveGame;

// XXX TODO this must contain (and calculate in its constructor) the effect on the gamestate.
// This includes:
//  * Losing players' token loss
//  * Honor gained by winning players' team
pub struct Outcome {}

struct DeclaredAttack {
    attackers: Vec<String>,
    defenders: Vec<String>,
    def_power: PowerType
}

// Initiates an attack, returning a closure over the data necessary to perform the next step of the
// attack.
fn declare_attack(attacker: &str,
                 defender: &str,
                 def_power: PowerType)
                 // Note: could use unstable feature:
                 // http://www.integer32.com/2017/02/02/stupid-tricks-with-higher-order-functions.html
                 -> AddingDefendersResult {

    let attack = DeclaredAttack {
        attackers: vec![String::from(attacker)],
        defenders: vec![String::from(defender)],
        def_power: def_power };

    // Q: Any way to infer the inner `AddDefender` type?
    AddingDefendersResult::AddDefender(AddDefender{attack: attack})
}

enum AddingDefendersResult {
    AddDefender(AddDefender),
    AddAttacker(AddAttacker)
}

enum AddingAttackersResult {
    AddAttacker(AddAttacker),
    Outcome(Outcome)
}

struct AddDefender {
    attack: DeclaredAttack
}

impl AddDefender {
    fn add(self, name: &str) -> AddingDefendersResult {
        // Q: Why no automatic wrapping as `Some`?
        // Maybe add a `derive` to impl deref as `Some`?
        add_defender_impl(self.attack, Some(name))
    }
    fn finalize_defense(self) -> AddAttacker {
        AddAttacker { attack: self.attack }
    }
}

struct AddAttacker {
    attack: DeclaredAttack
}

impl AddAttacker {
    fn add(self, name: &str) { add_attacker_impl(self.attack, Some(name)); }
    fn finalize_offense(self) -> Outcome {
        Outcome { /* XXX TODO */ }
    }
}

fn add_defender_impl(_attack: DeclaredAttack, _co_defender: Option<&str>) -> AddingDefendersResult {
    unimplemented!();
}

fn add_attacker_impl(_attack: DeclaredAttack, _co_attacker: Option<&str>) -> AddingAttackersResult {
    unimplemented!();
}
