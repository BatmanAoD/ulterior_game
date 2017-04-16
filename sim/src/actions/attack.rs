use gamestate::players::PowerType;
use gamestate::teams;
use gamestate::active::ActiveGame;

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


impl FnOnce for AddDefender {
    type Output = AddingDefendersResult;

    // XXX Impl `FnOnce` without the unsafe ABI stuff?
    extern "rust-call" fn call_once(&self, name: Option<&str>) -> Self::Output {
        AddDefender_impl(self.attack, name)
    }
}

struct AddAttacker {
    attack: DeclaredAttack
}


impl FnOnce for AddAttacker {
    type Output = AddingAttackersResult;

    extern "rust-call" fn call_once(&self, name: Option<&str>) -> Self::Output {
        AddAttacker_impl(self.attack, name)
    }
}

fn AddDefender_impl(attack: DeclaredAttack, co_defender: Option<&str>) -> AddingDefendersResult {
    unimplemented!();
}

fn AddAttacker_impl(attack: DeclaredAttack, co_attacker: Option<&str>) -> AddingAttackersResult {
    unimplemented!();
}
