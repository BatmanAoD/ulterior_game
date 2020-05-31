use crate::actions::attack::{Attack, AttackOutcome};
use crate::gamestate::players::{PName, Player, PlayerAttributePool, PlayersByName};
use crate::gamestate::teams::{TName, TeamsByName};
use crate::gamestate::with_history::{GameHistory, HistoryNavigationErr};

use rand::seq::SliceRandom;

use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::fmt;
use std::fs;

pub struct ActiveGame(GameHistory);

impl ActiveGame {
    pub fn new(
        player_names: impl Iterator<Item = String>,
        team_names: impl ExactSizeIterator<Item = String>,
        mut attribute_pool: impl PlayerAttributePool,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let mut player_list = player_names.collect::<Vec<_>>();
        // Randomize player order
        player_list.shuffle(&mut rng);
        let players_per_team = player_list.len() / team_names.len();
        let mut extra_players = player_list.len() % team_names.len();
        let mut teams: TeamsByName = Default::default();
        for team in team_names {
            // Add an extra player to the first (players % teams) teams
            let num_players = if extra_players > 0 {
                extra_players -= 1;
                players_per_team + 1
            } else {
                players_per_team
            };

            let players_on_team = player_list.drain(..num_players);

            teams.add(
                &team,
                PlayersByName::from(&team, players_on_team, &mut attribute_pool),
            );
        }
        assert!(player_list.is_empty());
        assert!(attribute_pool.is_empty());
        ActiveGame(GameHistory::starting_with(teams))
    }

    pub fn current_state(&self) -> &TeamsByName {
        self.0.current_state()
    }

    pub fn preview(&self, attack: Attack) -> AttackOutcome {
        attack.outcome(self.current_state())
    }

    pub fn apply_attack(&mut self, attack: Attack) {
        let outcome = self.preview(attack);
        self.apply_attack_outcome(outcome);
    }

    pub fn apply_attack_outcome(&mut self, attack: AttackOutcome) {
        self.0.apply_attack(attack);
    }

    pub fn undo_last_attack(&mut self) -> Result<(), HistoryNavigationErr> {
        self.0.undo_last_attack()
    }

    pub fn player_by_name(&self, name: &str) -> Option<(PName, TName)> {
        self.current_state().player_by_name(name)
    }

    pub fn player_data(&self, player: &PName) -> &Player {
        self.current_state().player_data(player)
    }

    pub fn player_mut(&mut self, player: &PName) -> &mut Player {
        self.0.current_mut().player_mut(player)
    }

    pub fn players(&self) -> impl Iterator<Item = &Player> {
        self.current_state().players()
    }

    pub fn players_mut(&mut self) -> impl Iterator<Item = &mut Player> {
        self.0.current_mut().players_mut()
    }

    pub fn pretty_player<'a>(&self, name: &'a PName) -> String {
        self.current_state().pretty_player(name)
    }

    pub fn pretty_players<'a>(&self, names: impl Iterator<Item = &'a PName>) -> String {
        self.current_state().pretty_players(names)
    }

    // Returns a mapping from the player name to the name of the file created for them
    pub fn write_roles<P: AsRef<Path>>(&self, d: P) -> BTreeMap<PName, String> {
        let dir = d.as_ref();
        assert!(dir.is_dir(), "'write_roles' given non-dir argument");
        let mut players_to_files: BTreeMap<_,_> = Default::default();
        for player in self.players() {
            let mut hasher = DefaultHasher::new();
            player.name.hash(&mut hasher);
            let basename = format!("{:x}", hasher.finish());
            // The `basename` should not include the extension, because GH Pages
            // will not render the content if the URL includes the extension
            let fpath = dir.join(basename.clone() + ".md");
            // TODO is it important to put the player name in the file?
            fs::write(fpath, &player.format_role()).expect("Could not write role file!");
            players_to_files.insert(player.name.to_owned(), basename);
        }
        players_to_files
    }
}

impl fmt::Display for ActiveGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Game state:")?;
        writeln!(f, "{}", self.current_state())
    }
}
