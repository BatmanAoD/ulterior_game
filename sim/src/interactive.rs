use ultlib::gamestate::{active::ActiveGame,builder::Setup};

use shrust::{ExecError, Shell, ShellIO};
use std::io::prelude::*;

pub fn run() {
    let mut setup = Setup::new_game();
    setup_teams(&mut setup);
    // TODO throughout, it would be nice to have more readable printing.
    // Eventually, there will need to be a way to show information to certain
    // players but not others.
    println!("{:?}", &setup);
    // TODO go back to `setup_teams` if adding a player returns TwoFewPlayers
    let /*mut*/ game = add_players(setup);
    println!("{:#?}", &game);
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

fn prompt(mut shell: Shell<&mut Setup>) {
    let mut io = ShellIO::default();
    shell.print_help(&mut io).unwrap();
    shell.run_loop(&mut io);
}