use itertools::{Itertools, Either};
use rand::Rng;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::PowerType;
use crate::gamestate::players::Player;
use crate::gamestate::players::PName;
// use crate::gamestate::teams;

#[derive(Debug)]
pub struct Attack {
    attackers: Vec<PName>,
    defenders: Vec<PName>,
    def_power: PowerType,
    att_power: PowerType,
}

impl Attack {
    fn determine_losers<'a>(self, state: &'a mut ActiveGame) -> (Vec<&'a mut Player>, PowerType) {
        // Q: Is there a way to do this categorization into *mutable* chunks in a way that will
        // satisfy the borrow checker?
        let (mut attackers, others): (Vec<_>, Vec<_>) = state
            .players_mut()
            .partition_map(|p| {
                if self.attackers.contains(&p.name) {
                    Either::Left(&mut p)
                } else {
                    Either::Right(&mut p)
                }
            });
        let (mut defenders, others) = others.iter_mut()
            .partition_map(|p| {
                if self.defenders.contains(&p.name) {
                    Either::Left(&mut p)
                } else {
                    Either::Right(&mut p)
                }
            });
        let attack_strength: i16 =
            attackers.iter()
            // Q: is explicit casting the right way to avoid overflow here?
            .map(|a| a.strength(self.att_power) as i16)
            .sum();
        let mut defenders: Vec<&mut Player> = self.defenders.iter()
            .map(
                |p| state.find_player_mut(p))
            .collect();
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
pub struct DeclaredAttack {
    attackers: Vec<PName>,
    defenders: Vec<PName>,
    def_power: PowerType,
    state: &ActiveGame,
}

impl DeclaredAttack {
    // Initiates an attack, returning a closure over the data necessary to perform the next step of the
    // attack.
    // TODO: instead of Option, use `Result` indicating which `str` wasn't found
    pub fn declare(state: &ActiveGame, attacker: &str, defender: &str, def_power: PowerType) -> Option<AddDefender> {
        let attack = DeclaredAttack {
            attackers: vec![state.get_pname(attacker)?.to_owned()],
            defenders: vec![state.get_pname(defender)?.to_owned()],
            def_power,
            state,
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
    pub fn add(mut self, name: &str) -> Result<Self, Self> {
        // TODO If defender already exists, Err
        // TODO warn if defender is on attacker's team?
        if let Some(pname) = self.attack.state.get_pname(name) {
            self.attack.defenders.push(pname.to_owned());
            Ok(Self)
        } else {
            Err(self)
        }
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
        // TODO what to do if attacker already exists?
        // TODO what to do if attacker is on `defenders` list?
        // TODO warn if attacker is on defender's team?
        self.attack.attackers.push(name.to_owned());
        self
    }
    pub fn finalize_offense(self) -> Attack {
        self.attack.finalize(self.att_power)
    }
}
