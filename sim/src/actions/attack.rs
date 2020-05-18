use maplit::btreeset;
use quick_error::quick_error;
use rand::Rng;
use std::collections::BTreeSet;
use std::fmt;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::{PName, Player, PowerType};
use crate::gamestate::teams::TName;

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
        self.players
            .iter()
            // Q: is explicit casting a reasonable way to avoid overflow here?
            .map(|a| a.strength(self.power_type) as i16)
            .sum()
    }
}

impl Attack {
    // TODO: It would be nice to split this into a pure function, `outcome`, and
    // a method on the output of that function, `apply`, that would actually
    // perform the operation. This would permit "previewing" the results of the attack.
    // However, since `state` is borrowed to construct `combatants_by_ref`, this
    // might not be possible as-is.
    pub fn apply(self, state: &mut ActiveGame) {
        let (attackers, defenders) = self.combatants_by_ref(state);
        let attack_strength = attackers.strength();
        let defense_strength = defenders.strength();
        // TODO DESIGN - should ties, or near-ties, be resolved w/out loss of
        // power or gain of honor?
        let attack_succeeds = attack_strength + self.attack_bonus() > defense_strength;
        let (losers, winning_team, honor_won) = if attack_succeeds {
            (defenders, self.attackers.for_team, defense_strength)
        } else {
            (attackers, self.defenders.for_team, attack_strength)
        };
        for loser in losers.players.into_iter() {
            loser.lose_power(losers.power_type);
        }
        state.teams.gain_honor(&winning_team, honor_won);
    }
    fn combatants_by_ref<'a>(
        &self,
        state: &'a mut ActiveGame,
    ) -> (CombatantRefs<'a>, CombatantRefs<'a>) {
        let (attackers, others): (Vec<_>, Vec<_>) = state
            .players_mut()
            .partition(|p| self.attackers.names.contains(&p.name));
        let defenders: Vec<_> = others
            .into_iter() // We move the existing `&mut`s rather than taking `&mut &mut`
            .filter(|p| self.defenders.names.contains(&p.name))
            .collect();
        (
            CombatantRefs {
                players: attackers,
                power_type: self.attackers.power_type,
            },
            CombatantRefs {
                players: defenders,
                power_type: self.defenders.power_type,
            },
        )
    }
    fn attack_bonus(&self) -> i16 {
        self.attackers
            .power_type
            .relative_advantage(self.defenders.power_type)
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
    // TODO DESIGN: Let attacker declare which team they're fighting for?
    pub fn declare<'g>(
        state: &'g ActiveGame,
        attacker: &str,
        defender: &str,
        def_power: PowerType,
    ) -> Result<AddDefender<'g>, DummyError> {
        let (attacker_name, att_team) = state.player_by_name(attacker).ok_or(DummyError::Dummy)?;
        let (defender_name, def_team) = state.player_by_name(defender).ok_or(DummyError::Dummy)?;
        Ok(AddDefender {
            attack: DeclaredAttack {
                attackers: btreeset! {attacker_name},
                att_team,
                defenders: btreeset! {defender_name},
                def_team,
                def_power,
                state,
            },
        })
    }

    pub fn finalize(self, att_power: PowerType) -> Attack {
        Attack {
            attackers: NamedCombatants {
                names: self.attackers,
                power_type: att_power,
                for_team: self.att_team,
            },
            defenders: NamedCombatants {
                names: self.defenders,
                power_type: self.def_power,
                for_team: self.def_team,
            },
        }
    }
}

impl<'a> fmt::Display for DeclaredAttack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO improve this
        writeln!(f,
"Attackers: representing team {}; combatants: {:?}
Defenders (power: {:?}): representing team {}; combatants: {:?}",
            self.att_team.0, self.attackers, self.def_power, self.def_team.0, self.defenders)
    }
}

pub struct AddDefender<'a> {
    pub attack: DeclaredAttack<'a>,
}

impl<'a> fmt::Display for AddDefender<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.attack.fmt(f)
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum DummyError {
        Dummy {}  // XXX TEMP
    }
}

impl<'a> AddDefender<'a> {
    pub fn add(&mut self, name: &str) -> Result<(), DummyError> {
        // TODO warn if defender is on attacker's team?
        if let Some((pname, _)) = self.attack.state.player_by_name(name) {
            if self.attack.defenders.insert(pname) {
                return Ok(());
            }
        }
        Err(DummyError::Dummy)
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
    pub attack: DeclaredAttack<'a>,
    att_power: PowerType,
}

impl<'a> fmt::Display for AddAttacker<'a> {
    // TODO improve this
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Attackers will use {:?} power.", self.att_power)?;
        self.attack.fmt(f)
    }
}

impl<'a> AddAttacker<'a> {
    pub fn add(&mut self, name: &str) -> Result<(), DummyError> {
        // TODO what to do if attacker is on `defenders` list?
        // TODO warn if attacker is on defender's team?
        if let Some((pname, _)) = self.attack.state.player_by_name(name) {
            if self.attack.defenders.contains(&pname) {
                return Err(DummyError::Dummy)
            }
            if self.attack.attackers.insert(pname) {
                return Ok(());
            }
        }
        Err(DummyError::Dummy)
    }

    pub fn add_or_panic(mut self, name: &str) -> Self {
        self.add(name).unwrap();
        self
    }

    pub fn finalize_offense(self) -> Attack {
        self.attack.finalize(self.att_power)
    }
}
