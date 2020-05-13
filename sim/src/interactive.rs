use ultlib::{actions, gamestate};

use shrust::{ExecError, Shell, ShellIO};
use std::io::prelude::*;

pub fn run() {
    let game = gamestate::builder::Setup::new_game();
    let mut shell = Shell::new(game);
    shell.new_command("team", "Add a new team", 1, |io, game, s| {
        game.add_team(s[0])
            .map_err(|e| ExecError::Other(Box::new(e)))?;
        writeln!(io, "{:?}", &game)?;
        Ok(())
    });
    shell.new_command_noargs(
        "done",
        "Finish adding teams and start adding players",
        |io, game| {
            writeln!(io, "{:?}", &game)?;
            Err(ExecError::Quit)
        },
    );

    shell.run_loop(&mut ShellIO::default());
    unimplemented!();
}
