use gamestate::players::PowerType;
use gamestate::teams;
use gamestate::active::ActiveGame;

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

    let mut attack = DeclaredAttack {
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
    // XXX TODO IMMEDIATE - did I want a generic 'call the struct itself like a function' deref for
    // some reason....?
    // If so, see:
    // https://dev.to/mindflavor/lets-build-zork-using-rust-1opm
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

// See above
//impl fnonce for addattacker {
//    type output = addingattackersresult;
//
//    extern "rust-call" fn call_once(&self, name: option<&str>) -> self::output {
//        addattacker_impl(self.attack, name)
//    }
//}

fn add_defender_impl(attack: DeclaredAttack, co_defender: Option<&str>) -> AddingDefendersResult {
    unimplemented!();
}

fn add_attacker_impl(attack: DeclaredAttack, co_attacker: Option<&str>) -> AddingAttackersResult {
    unimplemented!();
}
