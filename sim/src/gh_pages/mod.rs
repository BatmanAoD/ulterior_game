use crate::gamestate::active::ActiveGame;

use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

pub fn write_roles(game: &ActiveGame, name: String) {
    let game_dir = get_game_dir(&name);
    assert!(!game_dir.exists(), "Cannot re-use game name!");
    create_dir_all(&game_dir).expect("Cannot create directory");
    let game_base_url = get_base_url(&name);
    let written = game.write_roles(&game_dir);
    publish_files();
    println!("SECRET role information can be found at these URLs:");
    for (player, file) in written {
        println!("{:<15}{}{}", player, game_base_url, file);
    }
}

fn get_game_dir(name: &str) -> PathBuf {
    // XXX TEMP - assume current working dir is 'sim'
    format!("../docs/{}/roles", name).into()
}

fn get_base_url(name: &str) -> String {
    format!("https://batmanaod.github.io/ulterior_game/docs/{}/roles/", name)
}

fn publish_files() {
    // XXX TEMP
    println!("Press 'ENTER' when you have added the new files and pushed them.");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).expect("Could not read from stdin");
}