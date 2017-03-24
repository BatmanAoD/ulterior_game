extern crate rand;

mod gamestate;

fn main() {
    let mut players = gamestate::players::AllPlayers::new();
    players.add(gamestate::players::Player::new(
        String::from("Kyle"), String::from("Geats")));
    players.add(gamestate::players::Player::new(
        String::from("Laura"), String::from("Geats")));
    players.add(gamestate::players::Player::new(
        String::from("Brandon"), String::from("Geats")));
    players.add(gamestate::players::Player::new(
        String::from("Brad"), String::from("Geats")));
    players.add(gamestate::players::Player::new(
        String::from("Lauren"), String::from("Danes")));
    players.add(gamestate::players::Player::new(
        String::from("Annabelle"), String::from("Danes")));
    players.add(gamestate::players::Player::new(
        String::from("Aeris"), String::from("Danes")));
    players.add(gamestate::players::Player::new(
        String::from("Rosie"), String::from("Danes")));
    println!("Players are: {:?}", players);
}
