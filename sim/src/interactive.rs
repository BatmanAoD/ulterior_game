use ultlib::gamestate::{active::ActiveGame,builder::Setup,players::PowerType};
use ultlib::actions::attack::{AddDefender,AddAttacker,Attack,DeclaredAttack};

use quick_error::quick_error;
use shrust::{ExecError, Shell, ShellIO};
use std::io::prelude::*;
use std::io::BufReader;

// TODO throughout, most prompts only have one option. Use `Shell::set_default` instead of `new_command`.
// TODO let multiple teams/players/combatants be specified w/ one command

pub fn run() {
    play(setup());
}

quick_error! {
    #[derive(Debug)]
    enum InteractiveError {
        InvalidColorType {}
        PlayerDoesNotExist {}   // TODO which player(s)?
    }
}

fn play(mut game: ActiveGame) {
    let mut shell = Shell::new(&mut game);
    shell.new_command("attack", "Initiate a new attack; arg1: attacker, arg2: defender", 2, |io, game, s| {
        let declared = declare_attack(game, s, io)?;
        let attack = add_combatants(declared)?;

        writeln!(io, "About to apply: {:#?}", &attack)?;
        attack.apply(game);
        writeln!(io, "New game state: {:#?}", game)?;
        Ok(())
    });
    shell.set_prompt("Playing! Start a new attack, or quit: ".into());

    prompt(shell);
    println!("Final game state: {:#?}", &game);
}

fn setup() -> ActiveGame {
    let mut setup = Setup::new_game();
    setup_teams(&mut setup);
    // TODO throughout, it would be nice to have more readable printing.
    // Eventually, there will need to be a way to show information to certain
    // players but not others.
    println!("{:?}", &setup);
    // TODO go back to `setup_teams` if adding a player returns TwoFewPlayers
    let game = add_players(setup);
    println!("{:#?}", &game);
    return game
}

fn setup_teams(game: &mut Setup) {
    let mut shell = Shell::new(game);
    shell.new_command("team", "Add a new team", 1, |io, game, s| {
        game.add_team(s[0])
            .map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "{:?}", &game)?;
        Ok(())
    });
    shell.set_prompt("Add new team, or 'quit' to begin adding players:".into());

    prompt(shell);
}

fn add_players(mut game: Setup) -> ActiveGame {
    let mut shell = Shell::new(&mut game);
    shell.new_command("player", "Add a new player", 1, |io, game, s| {
        game.add_player(s[0])
            .map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "{:?}", &game)?;
        Ok(())
    });
    shell.set_prompt(
        "Add new player, or 'quit' to start the game (told you this implementation was rough):"
            .into(),
    );

    prompt(shell);
    game.finalize().expect("Could not initialize game")
}

fn declare_attack<'a>(game: &'a ActiveGame, s: &[&str], mut io: &mut ShellIO) -> Result<AddDefender<'a>, ExecError> {
    if game.player_by_name(s[0]).is_none() || game.player_by_name(s[1]).is_none() {
        return Err(ExecError::Other(Box::new(InteractiveError::PlayerDoesNotExist)))
    }

    writeln!(io, "Choose attack color (red > green > blue):")?;
    let mut reader = BufReader::new(&mut io);
    let mut color_input = String::new();
    reader.read_line(&mut color_input)?;
    let power_type = match color_input.trim().to_lowercase().as_str() {
        "red" => PowerType::Red,
        "green" => PowerType::Green,
        "blue" => PowerType::Blue,
        _ => return Err(ExecError::Other(Box::new(InteractiveError::InvalidColorType)))
    };
    
    DeclaredAttack::declare(
        &game,
        s[0],
        s[1],
        power_type,
    ).map_err(|e| ExecError::Other(Box::new(e)))
}

fn add_combatants(declared: AddDefender) -> Result<Attack, ExecError> {
    let defense_ready = add_defenders(declared)?;
    add_attackers(defense_ready)
}

fn add_defenders<'a>(mut declared: AddDefender<'a>) -> Result<AddAttacker<'a>, ExecError> {
    let mut shell = Shell::new(&mut declared);
    shell.new_command("defender", "Add a defender", 1, |_io, declared, s| {
        declared.add(s[0]).map_err(|e| ExecError::Other(Box::new(e)))
    });
    let prompt_str = format!("Attack: {:#?}\nAdd defender or quit to add more attackers:", shell.data().attack);
    shell.set_prompt(prompt_str);

    prompt(shell);

    Ok(declared.finalize_defense())
}

fn add_attackers(mut declared: AddAttacker) -> Result<Attack, ExecError> {
    let mut shell = Shell::new(&mut declared);
    shell.new_command("attacker", "Add an attacker", 1, |_io, declared, s| {
        declared.add(s[0]).map_err(|e| ExecError::Other(Box::new(e)))
    });
    let prompt_str = format!("Attack: {:#?}\nAdd attacker or quit to resolve the attack:", shell.data().attack);
    shell.set_prompt(prompt_str);

    prompt(shell);

    Ok(declared.finalize_offense())
}

fn prompt<T>(mut shell: Shell<T>) {
    let mut io = ShellIO::default();
    shell.print_help(&mut io).unwrap();
    shell.run_loop(&mut io);
}