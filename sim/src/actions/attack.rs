use maplit::btreeset;
use rand::Rng;
use std::collections::BTreeSet;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
use crate::gamestate::players::Player;
use crate::gamestate::players::PName;
use crate::gamestate::teams::TName;
// use crate::gamestate::teams;

#[derive(Debug)]
pub struct Attack {
    attackers: NamedCombatants,
    defenders: NamedCombatants,
}

#[derive(Debug)]
struct NamedCombatants {
    names: BTreeSet<PName>,
    power_type: PowerType,
    for_team: TName,
}

#[derive(Debug)]
struct CombatantRefs<'a> {
    players: Vec<&'a mut Player>,
    power_type: PowerType,
}

impl<'a> CombatantRefs<'a> {
    fn strength(&self) -> i16 {
        self.players.iter()
            // Q: is explicit casting the right way to avoid overflow here?
            .map(|a| a.strength(self.power_type) as i16)
            .sum()
    }
}

#[derive(Debug)]
struct AttackOutcome<'a> {
    state: &'a mut ActiveGame,
    losers: CombatantRefs<'a>,
    winning_team: TName,
    honor_won: i16,
}

impl<'a> AttackOutcome<'a> {
    fn apply(self) {
        for loser in self.losers.players.into_iter() {
            loser.lose_power(self.losers.power_type);
        }
        self.state.teams.gain_honor(&self.winning_team, self.honor_won)
    }
}

impl Attack {
    pub fn apply(self, state: &mut ActiveGame) {
        let outcome = self.outcome(state).apply();
    }
    fn outcome<'a>(self, state: &'a mut ActiveGame) -> AttackOutcome<'a> {
        let (attackers, defenders) = self.combatants_by_ref(state);
        let attack_strength = attackers.strength();
        let defense_strength = defenders.strength();
        let attack_succeeds = attack_strength + self.attack_bonus() > defense_strength;
        let losers: CombatantRefs<'a>;
        let winning_team: TName;
        let honor_won: i16;
        if attack_succeeds {
            losers = defenders;
            winning_team = self.attackers.for_team;
            honor_won = defense_strength;
        } else {
            losers = attackers;
            winning_team = self.defenders.for_team;
            honor_won = attack_strength;
        }
        AttackOutcome { state, losers, winning_team, honor_won }
    }
    fn combatants_by_ref<'a>(&self, state: &'a mut ActiveGame) -> (CombatantRefs<'a>, CombatantRefs<'a>) {
        let (attackers, others): (Vec<_>, Vec<_>) = state
            .players_mut()
            .partition(|p| self.attackers.names.contains(&p.name));
        let defenders: Vec<_> = others
            .into_iter()    // We move the existing `&mut`s rather than taking `&mut &mut`
            .filter(|p| self.defenders.names.contains(&p.name))
            .collect();
        (
            CombatantRefs{ players: attackers, power_type: self.attackers.power_type},
            CombatantRefs{ players: defenders, power_type: self.defenders.power_type},
        )
    }
    fn attack_bonus(&self) -> i16 {
        self.attackers.power_type.relative_advantage(self.defenders.power_type)
    }
}

#[derive(Debug)]
pub struct DeclaredAttack<'a> {
    attackers: BTreeSet<PName>,
    att_team: TName,
    defenders: BTreeSet<PName>,
    def_team: TName,
    def_power: PowerType,
    state: &'a ActiveGame,
}

impl<'a> DeclaredAttack<'a> {
    // Initiates an attack, returning a closure over the data necessary to perform the next step of the
    // attack.
    // XXX TODO: instead of Option, use `Result` indicating which `str` wasn't found
    pub fn declare<'g>(
        state: &'g ActiveGame,
        attacker: &str,
        defender: &str,
        def_power: PowerType,
    ) -> Option<AddDefender<'g>> {
        Some(AddDefender {
            attack: DeclaredAttack {
                attackers: btreeset!{state.get_pname(attacker)?.to_owned()},
                defenders: btreeset!{state.get_pname(defender)?.to_owned()},
                def_power,
                state,
            }
        })
    }

    pub fn finalize(self, att_power: PowerType) -> Attack {
        Attack {
            attackers: NamedCombatants{ names: self.attackers, power_type: att_power, for_team: self.att_team },
            defenders: NamedCombatants{ self.defenders, self.def_power, for_team: self.def_team },
        }
    }
}

pub struct AddDefender<'a> {
    attack: DeclaredAttack<'a>,
}

#[derive(Debug)]
pub struct DummyError {}    // XXX TEMP

impl<'a> AddDefender<'a> {
    pub fn add(&mut self, name: &str) -> Result<bool, DummyError> {
        // TODO If defender already exists, Err
        // TODO warn if defender is on attacker's team?
        if let Some(pname) = self.attack.state.get_pname(name) {
            Ok(self.attack.defenders.insert(pname.to_owned()))
        } else {
            Err(DummyError{})
        }
    }

    pub fn add_or_panic(mut self, name: &str) -> Self {
        self.add(name).unwrap();
        self
    }

    pub fn finalize_defense(self) -> AddAttacker<'a> {
        let mut rng = rand::thread_rng();
        AddAttacker {
            attack: self.attack,
            att_power: rng.gen(),
        }
    }
}

pub struct AddAttacker<'a> {
    attack: DeclaredAttack<'a>,
    att_power: PowerType,
}

impl<'a> AddAttacker<'a> {
    pub fn add(&mut self, name: &str) -> Result<bool, DummyError> {
        // TODO what to do if attacker already exists?
        // TODO what to do if attacker is on `defenders` list?
        // TODO warn if attacker is on defender's team?
        if let Some(pname) = self.attack.state.get_pname(name) {
            Ok(self.attack.attackers.insert(pname.to_owned()))
        } else {
            Err(DummyError{})
        }
    }

    pub fn add_or_panic(mut self, name: &str) -> Self {
        self.add(name).unwrap();
        self
    }

    pub fn finalize_offense(self) -> Attack {
        self.attack.finalize(self.att_power)
    }
}
