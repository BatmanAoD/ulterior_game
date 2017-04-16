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
                 // Q: easier way to declare this?
                 // Unstable feature:
                 // http://www.integer32.com/2017/02/02/stupid-tricks-with-higher-order-functions.html
                 -> Box<Fn<Option<&str> -> Fn<Option<&str> -> Outcome>>> {
    let mut attack = DeclaredAttack {
        attackers: vec![attacker],
        defenders: vec![defender],
        def_power: PowerType };

    |co_defender: Option<&str>| AddDefender(attack, co_defender)
}

fn AddDefender(attack: &mut DeclaredAttack, co_defender: Option<&str>) -> Box<AddAttacker> {
    unimplemented!();
}

fn AddAttacker(co_attacker: Option<&str>) -> Outcome {
    unimplemented!();
}
