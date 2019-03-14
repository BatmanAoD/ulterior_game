use itertools::{Itertools, Either};
use maplit::btreeset;
use rand::Rng;
use std::collections::BTreeSet;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
use crate::gamestate::players::Player;
use crate::gamestate::players::PName;
// use crate::gamestate::teams;

#[derive(Debug)]
pub struct Attack {
    attackers: BTreeSet<PName>,
    defenders: BTreeSet<PName>,
    def_power: PowerType,
    att_power: PowerType,
}

impl Attack {
    fn determine_losers<'a>(self, state: &'a mut ActiveGame) -> (Vec<&'a mut Player>, PowerType) {
        // Q: Is there a way to do this categorization into *mutable* chunks in a way that will
        // satisfy the borrow checker?
        let (attackers, mut others): (Vec<_>, Vec<_>) = state
            .players_mut()
            .partition_map(|p| {
                if self.attackers.contains(&p.name) {
                    Either::Left(p)
                } else {
                    Either::Right(p)
                }
            });
        let (defenders, _others): (Vec<&mut Player>, Vec<_>) = others.iter_mut()
            // XXX Q: why does `partition_map` produce `&mut &mut _` values here?
            // Attempting to manually dereference these with `*` doesn't work,
            // apparently because that attempts to *copy* the `&mut`, which of
            // course would be wrong.
            .partition_map(|p| {
                if self.defenders.contains(&p.name) {
                    Either::Left(p)
                } else {
                    Either::Right(p)
                }
            });
        let attack_strength: i16 =
            attackers.iter()
            // Q: is explicit casting the right way to avoid overflow here?
            .map(|a| a.strength(self.att_power) as i16)
            .sum();
        let defense_strength: i16 =
            defenders.iter()
            .map(|d| d.strength(self.def_power) as i16)
            .sum();
        if attack_strength > defense_strength {
            (defenders, self.def_power)
        } else {
            (attackers, self.att_power)
        }
    }
    pub fn apply(self, state: &mut ActiveGame) {
        let (losers, p_type) = self.determine_losers(&mut state);
        for loser in losers.iter() {
            loser.lose_power(p_type);
            // XXX TODO: winning players should win honor!
            unimplemented!();
        }
    }
}

#[derive(Debug)]
pub struct DeclaredAttack<'a> {
    attackers: BTreeSet<PName>,
    defenders: BTreeSet<PName>,
    def_power: PowerType,
    state: &'a ActiveGame,
}

impl<'a> DeclaredAttack<'a> {
    // Initiates an attack, returning a closure over the data necessary to perform the next step of the
    // attack.
    // TODO: instead of Option, use `Result` indicating which `str` wasn't found
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
            attackers: self.attackers,
            defenders: self.defenders,
            def_power: self.def_power,
            att_power,
        }
    }
}

pub struct AddDefender<'a> {
    attack: DeclaredAttack<'a>,
}

#[derive(Debug)]
pub struct DummyError {}    // XXX TEMP

impl<'a> AddDefender<'a> {
    pub fn add(mut self, name: &str) -> Result<bool, DummyError> {
        // TODO If defender already exists, Err
        // TODO warn if defender is on attacker's team?
        if let Some(pname) = self.attack.state.get_pname(name) {
            Ok(self.attack.defenders.insert(pname.to_owned()))
        } else {
            Err(DummyError{})
        }
    }

    pub fn add_or_panic(mut self, name: &str) -> &mut Self {
        self.add(name).unwrap();
        &mut self
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
    pub fn add(mut self, name: &str) -> Result<bool, DummyError> {
        // TODO what to do if attacker already exists?
        // TODO what to do if attacker is on `defenders` list?
        // TODO warn if attacker is on defender's team?
        if let Some(pname) = self.attack.state.get_pname(name) {
            Ok(self.attack.attackers.insert(pname.to_owned()))
        } else {
            Err(DummyError{})
        }
    }

    pub fn add_or_panic(mut self, name: &str) -> &mut Self {
        self.add(name).unwrap();
        &mut self
    }

    pub fn finalize_offense(self) -> Attack {
        self.attack.finalize(self.att_power)
    }
}
