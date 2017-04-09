extern crate rand;

mod gamestate;

fn main() {
    let mut builder = gamestate::builder::Setup::new_game();
    builder.add_team_or_panic("Geats");
    builder.add_team_or_panic("Danes");
    builder.add_player_or_panic("Kyle");
    // builder.add_team_or_panic("Danes");
    builder.add_player_or_panic("Laura");
    builder.add_player_or_panic("Brandon");
    builder.add_player_or_panic("Brad");
    builder.add_player_or_panic("Lauren");
    builder.add_player_or_panic("Annabelle");
    builder.add_player_or_panic("Aeris");
    builder.add_player_or_panic("Rosie");
    println!("Start-of-game setup: {:#?}", builder.finalize());
}
