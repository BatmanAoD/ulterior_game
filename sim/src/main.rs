extern crate rand;

mod gamestate;

fn main() {
    let mut builder = gamestate::builder::Setup::new_game();
    builder.add_team("Geats");
    builder.add_team("Danes");
    builder.add_player("Kyle");
    builder.add_player("Laura");
    builder.add_player("Brandon");
    builder.add_player("Brad");
    builder.add_player("Lauren");
    builder.add_player("Annabelle");
    builder.add_player("Aeris");
    builder.add_player("Rosie");
    println!("Start-of-game setup: {:?}", builder.finalize());
}
