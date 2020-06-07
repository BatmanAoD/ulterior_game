mod interactive;

fn main() {
    interactive::run();
}

#[cfg(test)]
use ultlib::actions::attack::DeclaredAttack;
#[cfg(test)]
use ultlib::gamestate::{builder::Setup, power::PowerType};

#[test]
fn dummy_game() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Setup::new_game()
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
        .player_without_role_or_panic("Annabelle")
        .finalize()
        .unwrap();
    println!("Start-of-game setup: {}", &game);

    assert!(game.player_data(&game.player_by_name("Annabelle").unwrap().0).format_role() == "Few know their own destiny; you have yet to discover yours.");

    let attack = DeclaredAttack::declare(
        &game.current_state(),
        "Kyle",
        "Brandon",
        PowerType::Red,
    )
    .unwrap()
    .add_or_panic("Laura")
    .add_or_panic("Annabelle")
    .finalize_defense()
    .add_or_panic("Suzie")
    .finalize_offense();

    let attack_color = attack.attack_power();

    println!("Attack: {:#?}", &attack);
    game.apply_attack(attack);
    println!("After attack resolves: {}", &game);

    // Defenders should only lose the color they defended with; assists should
    // always lose power
    let brandon = game.player_data(&game.player_by_name("Brandon").unwrap().0);
    assert!(brandon.has_power(PowerType::Blue));
    assert!(brandon.has_power(PowerType::Green));
    let laura = game.player_data(&game.player_by_name("Laura").unwrap().0);
    assert!(laura.has_power(PowerType::Blue));
    assert!(laura.has_power(PowerType::Green));
    assert!(!laura.has_power(PowerType::Red));
    let annabelle = game.player_data(&game.player_by_name("Annabelle").unwrap().0);
    assert!(annabelle.has_power(PowerType::Blue));
    assert!(annabelle.has_power(PowerType::Green));
    assert!(!annabelle.has_power(PowerType::Red));

    let kyle = game.player_data(&game.player_by_name("Kyle").unwrap().0);
    let suzie = game.player_data(&game.player_by_name("Suzie").unwrap().0);
    for color in &[PowerType::Red, PowerType::Green, PowerType::Blue] {
        if *color != attack_color {
            // Attackers should only lose the color they attacked with
            assert!(kyle.has_power(*color));
            assert!(suzie.has_power(*color));
        }
        else {
            // assists should always lose power
            assert!(!suzie.has_power(*color));
        }
    }

    Ok(())
}
