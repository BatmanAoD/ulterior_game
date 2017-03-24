extern crate rand;

mod gamestate;

fn main() {
    let p = gamestate::players::Player::new(
        String::from("Kyle"), String::from("Geats"));
    let mut players = gamestate::players::AllPlayers::new();
    players.add(p);
    println!("Players are: {:?}", players);
}
