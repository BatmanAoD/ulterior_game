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
fn DeclareAttack(attacker: &str,
                 defender: &str,
                 def_power: PowerType)
                 // Note: could use unstable feature:
                 // http://www.integer32.com/2017/02/02/stupid-tricks-with-higher-order-functions.html
                 -> AddingDefendersResult {

    let mut attack = DeclaredAttack {
        attackers: vec![String::from(attacker)],
        defenders: vec![String::from(defender)],
        def_power: def_power };

    AddingDefendersResult::AddDefender{0.attack: attack}
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
    fn add(&mut self, name: &str) {
        AddDefender_impl(self.attack, name)
    }
    fn finalize_defense(self) {
        AddAttacker { 0.attack: self.attack }
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
    fn add(&mut self, name: &str) { AddAttacker_impl(self.attack, name); }
    fn finalize_offense(self) {
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

fn AddDefender_impl(attack: DeclaredAttack, co_defender: Option<&str>) -> AddingDefendersResult {
    unimplemented!();
}

fn AddAttacker_impl(attack: DeclaredAttack, co_attacker: Option<&str>) -> AddingAttackersResult {
    unimplemented!();
}
