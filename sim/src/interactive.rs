use ultlib::gamestate::{active::ActiveGame,builder::Setup,power::PowerType};
use ultlib::actions::attack::{AddDefender,AddAttacker,Attack,DeclaredAttack};

use quick_error::quick_error;
use shrust::{ExecError, Shell, ShellIO};
use std::io::prelude::*;
use std::io::BufReader;

// TODO throughout, it would be nice to have more readable printing.
// Eventually, there will need to be a way to show information to certain
// players but not others.

pub fn run() {
    let game = setup_game();
    println!("{}", &game);
    play(game);
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
    // TODO: since this is the only command, use `Shell::set_default` instead?
    shell.new_command("attack", "Initiate a new attack; arg1: attacker, arg2: defender", 2, |io, game, s| {
        let declared = declare_attack(game, s, io)?;
        let attack = add_combatants(declared)?;

        writeln!(io, "About to apply: {}", &attack)?;
        attack.apply(game);
        writeln!(io, "{}", game)?;
        Ok(())
    });
    shell.set_prompt("Playing! Start a new attack, or quit: ".into());

    prompt(shell);
    println!("Final game state: {}", &game);
}

fn setup_game() -> ActiveGame {
    let mut setup = Setup::new_game();
    let mut shell = Shell::new(&mut setup);
    shell.new_command("team", "Add a new team", 1, |io, setup, s| {
        setup.add_team(s[0])
            .map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "{:?}", &setup)?;
        Ok(())
    });
    shell.new_command("player", "Add a new player", 1, |io, setup, s| {
        setup.add_player(s[0])
            .map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "{:?}", &setup)?;
        Ok(())
    });
    shell.set_prompt("Add new team or player name, or 'quit' to finish setup:".into());

    prompt(shell);
    setup.finalize().expect("Could not initialize game")
}

fn declare_attack<'a>(game: &'a ActiveGame, s: &[&str], mut io: &mut ShellIO) -> Result<AddDefender<'a>, ExecError> {
    if game.player_by_name(s[0]).is_none() || game.player_by_name(s[1]).is_none() {
        return Err(ExecError::Other(Box::new(InteractiveError::PlayerDoesNotExist)))
    }

    writeln!(io, "Choose defense color (red > green > blue):")?;
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
    println!("Adding defenders to {}", &declared);
    let mut shell = Shell::new(&mut declared);
    shell.new_command("defender", "Add a defender", 1, |io, declared, s| {
        declared.add(s[0]).map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "Adding defenders to {}", declared)?;
        Ok(())
    });
    shell.set_prompt("Add defender or quit to add more attackers:".into());

    prompt(shell);

    Ok(declared.finalize_defense())
}

fn add_attackers(mut declared: AddAttacker) -> Result<Attack, ExecError> {
    println!("Adding attackers to {}", &declared);
    let mut shell = Shell::new(&mut declared);
    shell.new_command("attacker", "Add an attacker", 1, |io, declared, s| {
        declared.add(s[0]).map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "Adding attackers to {}", declared)?;
        Ok(())
    });
    shell.set_prompt("Add attacker or quit to resolve the attack:".into());

    prompt(shell);

    Ok(declared.finalize_offense())
}

fn prompt<T>(mut shell: Shell<T>) {
    let mut io = ShellIO::default();
    shell.print_help(&mut io).unwrap();
    shell.run_loop(&mut io);
}