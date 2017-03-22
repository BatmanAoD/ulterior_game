mod gamestate;

fn main() {
    let p = gamestate::players::Player {
        name: String::from("Kyle"),
        team: String::from("Geats"),
        // Q: Why can't rustc infer that the braces are initializing a 'Power' struct? I.e., why
        // not just `power: { ....`
        power: gamestate::players::Power {
            red: Some(2), blue: Some(4), green: Some(3)
        },
        role: None
    };
    println!("Player: {:?}", p);
}
