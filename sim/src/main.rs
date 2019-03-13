extern crate rand;

mod actions;
mod gamestate;

fn main() {
    unimplemented!();
}

#[test]
fn dummy_game() {
    let mut builder = gamestate::builder::Setup::new_game();
    builder.add_team_or_panic("Geats");
    builder.add_team_or_panic("Danes");

    builder.add_player_or_panic("Kyle");
    builder.add_player_or_panic("Laura");
    builder.add_player_or_panic("Brandon");
    builder.add_player_or_panic("Suzie");
    builder.add_player_or_panic("Lauren");
    builder.add_player_or_panic("Annabelle");
    builder.add_player_or_panic("Aeris");
    builder.add_player_or_panic("Rosie");
    let mut game = builder.finalize().unwrap();
    println!("Start-of-game setup: {:#?}", &game);

    let attack = actions::attack::DeclaredAttack::declare(
            "Kyle", "Brandon", gamestate::players::PowerType::Red)
        .add("Laura")
        .add("Annabelle")
        .finalize_defense()
        .add("Suzie")
        .finalize_offense();

    println!("Attack: {:#?}", &attack);
    attack.apply(&mut game);
}
