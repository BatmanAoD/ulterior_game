use itertools::Itertools;
use quick_error::quick_error;
use rand::Rng;
use std::collections::BTreeSet;
use std::fmt;

use crate::gamestate::active::ActiveGame;
use crate::gamestate::players::{PName, Player};
use crate::gamestate::power::PowerType;
use crate::gamestate::teams::TName;

#[derive(Debug)]
pub struct Attack {
    attackers: NamedCombatants,
    defenders: NamedCombatants,
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO improve this
        write!(f,
            // Formatting `NamedCombatants` will insert newlines
            "Attackers: {}Defenders {}",
            self.attackers, self.defenders)
    }
}

#[derive(Debug)]
struct NamedCombatants {
    primary: PName,
    assists: BTreeSet<PName>,
    power_type: PowerType,
    for_team: TName,
}

impl fmt::Display for NamedCombatants {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
            "{} ({}); representing team {}; combatants: {}",
            self.primary, self.power_type, self.for_team.0, self.assists.iter().join(", "))
    }
}

#[derive(Debug)]
struct CombatantRefs<'a> {
    primary: &'a mut Player,
    assists: Vec<&'a mut Player>,
    power_type: PowerType,
}

impl<'a> CombatantRefs<'a> {
    fn strength(&self) -> i16 {
        self.primary.strength(self.power_type) as i16
        + self.assists
            .iter()
            // Q: is explicit casting a reasonable way to avoid overflow here?
            .map(|a| a.strength(self.power_type) as i16)
            .sum::<i16>()
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
        let (losers, win_assists, winning_team, honor_won) = if attack_succeeds {
            (defenders, attackers.assists, self.attackers.for_team, defense_strength)
        } else {
            (attackers, defenders.assists, self.defenders.for_team, attack_strength)
        };
        // The primary combatant always loses their token.
        losers.primary.lose_power(losers.power_type);
        // Assists on *both* sides of the combat lose their tokens.
        for assist in losers.assists.into_iter().chain(win_assists.into_iter()) {
            assist.lose_power(losers.power_type);
        }
        state.gain_honor(&winning_team, honor_won);
    }
    fn combatants_by_ref<'a>(
        &self,
        state: &'a mut ActiveGame,
    ) -> (CombatantRefs<'a>, CombatantRefs<'a>) {
        // Q: This is... pretty ugly. Is there a more elegant way? Slice patterns, maybe?
        let (v_primary_attacker, others): (Vec<_>, Vec<_>) = state
            .players_mut()
            .partition(|p| p.name == self.attackers.primary);
        let (attacker_assists, others): (Vec<_>, Vec<_>) = others
            .into_iter()
            .partition(|p| self.attackers.assists.contains(&p.name));
        let (v_primary_defender, others): (Vec<_>, Vec<_>) = others
            .into_iter()
            .partition(|p| p.name == self.defenders.primary);
        let defender_assists: Vec<_> = others
            .into_iter() // We move the existing `&mut`s rather than taking `&mut &mut`
            .filter(|p| self.defenders.assists.contains(&p.name))
            .collect();
        let primary_attacker = v_primary_attacker.into_iter().next().expect("Could not find primary attacker data");
        let primary_defender = v_primary_defender.into_iter().next().expect("Could not find primary defender data");
        (
            CombatantRefs {
                primary: primary_attacker,
                assists: attacker_assists,
                power_type: self.attackers.power_type,
            },
            CombatantRefs {
                primary: primary_defender,
                assists: defender_assists,
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
    initial_attacker: PName,
    attacker_assists: BTreeSet<PName>,
    att_team: TName,
    targeted_defender: PName,
    defender_assists: BTreeSet<PName>,
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
    ) -> Result<AddDefender<'g>, InvalidAttackErr> {
        let (attacker_name, att_team) = state.player_by_name(attacker).ok_or(InvalidAttackErr::CombatantNotFound)?;
        let (defender_name, def_team) = state.player_by_name(defender).ok_or(InvalidAttackErr::CombatantNotFound)?;
        if !state.player_data(defender_name).has_power(def_power) {
            return Err(InvalidAttackErr::CombatantMissingPowerType)
        }
        Ok(AddDefender {
            attack: DeclaredAttack {
                initial_attacker: attacker_name,
                attacker_assists: Default::default(),
                att_team,
                targeted_defender: defender_name,
                defender_assists: Default::default(),
                def_team,
                def_power,
                state,
            },
        })
    }

    pub fn finalize(self, att_power: PowerType) -> Attack {
        Attack {
            attackers: NamedCombatants {
                primary: self.initial_attacker,
                assists: self.attacker_assists,
                power_type: att_power,
                for_team: self.att_team,
            },
            defenders: NamedCombatants {
                primary: self.targeted_defender,
                assists: self.defender_assists,
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
"Attackers: representing team {}; combatants:
{} (initial attacker)
{}
Defenders (power: {}): representing team {}; combatants:
{} (targeted defender)
{}",
            self.att_team.0,
            self.state.pretty_player(&self.initial_attacker),
            self.state.pretty_players(self.attacker_assists.iter()),
            self.def_power,
            self.def_team.0,
            self.state.pretty_player(&self.targeted_defender),
            self.state.pretty_players(self.defender_assists.iter()),
        )
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
    pub enum InvalidAttackErr {
        CombatantNotFound {}
        CombatantMissingPowerType {}
        DuplicateCombatant {}
        AttackerAlreadyDefending {}
    }
}

impl<'a> AddDefender<'a> {
    pub fn add(&mut self, name: &str) -> Result<(), InvalidAttackErr> {
        // TODO warn if defender is on attacker's team?
        // TODO DESIGN - since assists sacrifice their tokens, should they be permitted to
        // pick a token to sacrifice?
        if let Some((pname, _)) = self.attack.state.player_by_name(name) {
            if !self.attack.state.player_data(&pname).has_power(self.attack.def_power) {
                return Err(InvalidAttackErr::CombatantMissingPowerType);
            }
            if !self.attack.defender_assists.insert(pname) {
                return Err(InvalidAttackErr::DuplicateCombatant);
            }
            Ok(())
        } else {
            Err(InvalidAttackErr::CombatantNotFound)
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
    pub attack: DeclaredAttack<'a>,
    att_power: PowerType,
}

impl<'a> fmt::Display for AddAttacker<'a> {
    // TODO improve this
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Attackers will use {} power.", self.att_power)?;
        self.attack.fmt(f)
    }
}

impl<'a> AddAttacker<'a> {
    pub fn add(&mut self, name: &str) -> Result<(), InvalidAttackErr> {
        // TODO warn if attacker is on defender's team?
        if let Some((pname, _)) = self.attack.state.player_by_name(name) {
            if !self.attack.state.player_data(&pname).has_power(self.attack.def_power) {
                return Err(InvalidAttackErr::CombatantMissingPowerType);
            }
            if pname == self.attack.targeted_defender || self.attack.defender_assists.contains(&pname) {
                return Err(InvalidAttackErr::AttackerAlreadyDefending);
            }
            if !self.attack.attacker_assists.insert(pname) {
                return Err(InvalidAttackErr::DuplicateCombatant)
            }
            Ok(())
        } else {
            Err(InvalidAttackErr::CombatantNotFound)
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
