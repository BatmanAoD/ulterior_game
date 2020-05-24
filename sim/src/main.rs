mod interactive;

fn main() {
    interactive::run();
}

#[cfg(test)]
use ultlib::{actions, gamestate};

#[test]
fn dummy_game() -> Result<(), actions::attack::InvalidAttackErr> {
    let mut game = gamestate::builder::Setup::new_game()
        .add_team_or_panic("Geats")
        .add_player_or_panic("Kyle")
        .add_player_or_panic("Laura")
        .add_team_or_panic("Danes")
        .add_player_or_panic("Brandon")
        .add_player_or_panic("Suzie")
        .add_player_or_panic("Lauren")
        .add_player_or_panic("Annabelle")
        .add_player_or_panic("Luna")
        .add_player_or_panic("Rosie")
        .finalize()
        .unwrap();
    println!("Start-of-game setup: {}", &game);

    let attack = actions::attack::DeclaredAttack::declare(
        &game.current_state(),
        "Kyle",
        "Brandon",
        gamestate::power::PowerType::Red,
    )
    .unwrap()
    .add_or_panic("Laura")
    .add_or_panic("Annabelle")
    .finalize_defense()
    .add_or_panic("Suzie")
    .finalize_offense();

    println!("Attack: {:#?}", &attack);
    game.apply_attack(attack);
    println!("After attack resolves: {}", &game);

    Ok(())
}
