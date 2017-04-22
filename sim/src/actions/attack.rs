use gamestate::players::PowerType;
use gamestate::teams;
use gamestate::active::ActiveGame;

// XXX TODO this must contain (and calculate in its constructor) the effect on the gamestate.
// This includes:
//  * Losing players' token loss
//  * Honor gained by winning players' team
pub struct Outcome {}

struct DeclaredAttack {
    attackers: Vec<&str>,
    defenders: Vec<&str>,
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
        attackers: vec![attacker],
        defenders: vec![defender],
        def_power: def_power };

    AddingDefendersResult::AddDefender{attack: attack}
}

enum AddingDefendersResult {
    AddDefender,
    AddAttacker
}

enum AddingAttackersResult {
    AddAttacker,
    Outcome
}

struct AddDefender {
    attack: DeclaredAttack
}

// TODO figure out why functional/closure-based approach didn't work
// (in particular, how to create functors w/out incurring the "ABI is subject to change" error for
// `impl` of `FnMut`?)

impl AddDefender {
    fn add(&mut self, name: &str) {
        AddDefender_impl(self.attack, name);
    }
    fn finalize_defense(self) {
        AddAttacker { attack: self.attack }
    }
}

// TODO: define custom traits inheriting from FnOnce?
// See http://stackoverflow.com/a/26071172/1858225 for possible syntax, although that's from about a
// year before the release of 1.0

//impl FnOnce for AddDefender {
//    type Output = AddingDefendersResult;
//
//    // XXX Impl `FnOnce` without the unsafe ABI stuff?
//    extern "rust-call" fn call_once(&self, name: Option<&str>) -> Self::Output {
//        AddDefender_impl(self.attack, name)
//    }
//}

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
