extern crate term_painter;

use db::types::*;
use db::data::*;
use db::pokemon_by_name;
use engine::Pokemon;
use self::term_painter::Style;
use self::term_painter::Painted;
use self::term_painter::Attr::{Plain, Bold};
use self::term_painter::ToStyle;
use self::term_painter::Color::{Black, White, Red, Blue, Yellow, Magenta, Green};

/// Coordinates a fight between two players and their pokemon
pub fn fight(player_one: &str, player_two: &str) {

    // Let both players choose their Pokemon
    let model_red = choose_pokemon(player_one);
    let mut poki_red = Pokemon::with_level(model_red, 5);

    let model_blue = choose_pokemon(player_two);
    let mut poki_blue = Pokemon::with_level(model_blue, 5);


    loop {
        fn check_dead(poki: &Pokemon) -> bool {
            if poki.is_alive() {
                false
            } else {
                println!(">>>>> {} fainted!", Bold.fg(Black).bg(White).paint(poki.name()));
                true
            }
        }

        // Print status
        println!(
            ">>>>> Status: {} has {} HP, {} has {} HP",
            paint_pokemon(poki_red.model()),
            paint_hp(poki_red.stats().hp),
            paint_pokemon(poki_blue.model()),
            paint_hp(poki_blue.stats().hp),
        );

        // Execute both attack
        if poki_red.stats().speed > poki_blue.stats().speed {
            // Red attacks blue
            execute_round(&poki_red, &mut poki_blue);
            if check_dead(&poki_blue) {
                break;
            }

            // BLue attacks red
            execute_round(&poki_blue, &mut poki_red);
            if check_dead(&poki_red) {
                break;
            }
        } else {
            // BLue attacks red
            execute_round(&poki_blue, &mut poki_red);
            if check_dead(&poki_red) {
                break;
            }

            // Red attacks blue
            execute_round(&poki_red, &mut poki_blue);
            if check_dead(&poki_blue) {
                break;
            }
        }
    }
}

/// Executes one round of one player:
///
/// - the player chooses one attack to execute
/// - the attack is excuted and the enemy's HP
fn execute_round(attacker: &Pokemon, defender: &mut Pokemon) {

    // Tell the user to choose an attack
    println!(
        ">>>>> {} is about to attack! Which move shall it execute?",
        paint_pokemon(attacker.model())
    );

    // Print a list of available attacks
    let num_attacks = attacker.model().attacks.len();
    for i in 0..num_attacks {
        println!("    {}: {}", i, paint_attack(attacker.model().attacks[i]));
    }
    println!("    !!! Please give me the attack ID:");

    // Read attack ID from the user
    let attack_id;
    loop {
        let input = read_usize();
        if input >= num_attacks {
            println!("    !!! There is no attack with index {}!", input);
        } else {
            attack_id = input;
            break;
        }
    }

    // Execute attack
    let attack = *attacker.model().attacks[attack_id];
    defender.endure_attack(attacker, attack);

    // Status update
    println!(
        ">>>>> {} uses {}! ({} has {} HP left)",
        paint_pokemon(attacker.model()),
        paint_attack(&attack),
        paint_pokemon(defender.model()),
        paint_hp(defender.stats().hp),
    );
}

/// Let's the player choose a Pokemon from the database.
fn choose_pokemon(player: &str) -> &'static PokemonModel {
    // Loop forever until the user has chosen a Pokemon
    loop {
        println!(
            "{}, please choose a Pokemon (or type '?' to get a complete list)",
            player,
        );

        let input = read_string();
        if input == "?" {
            print_pokemon_list();
        } else {
            // Try to find a Pokemon with the given name
            match pokemon_by_name(&input) {
                Some(poki) => return poki,
                None => {
                    println!("No pokemon with the name '{}' was found!", input);
                }
            }
        }
    }
}

/// Prints a list of all Pokemon in the database.
fn print_pokemon_list() {
    for poki in POKEDEX {
        // This strange formatter will print the pokemon ID with three digits,
        // filling in 0 from the left if necessary (#003).
        println!("#{:0>3} {}", poki.id, paint_pokemon(poki));
    }
}

/// Paint the HP in bold (if the console can manage that)
/// and make the text red if the HP fall under 6
fn paint_hp(life: u16) -> Painted<u16> {
    let color = match life {
        0 ... 5 => Bold.fg(Red),
        _ => Bold.fg(White).bg(Black),
    };

    color.paint(life)
}

/// Paint the attack name according to its type
fn paint_attack(att: &Attack) -> Painted<&str> {
    get_type_color(att.type_).paint(att.name)
}

/// Paint the pokemon name according to its type
fn paint_pokemon(poki: &'static PokemonModel) -> Painted<&str> {
    let color = match poki.type_ {
        PokemonType::One(t) | PokemonType::Two(t, _) => get_type_color(t),
    };

    color.paint(poki.name)
}

/// Choose Style according to type
fn get_type_color(t: Type) -> Style {
    match t {
        Type::Fire => Plain.fg(Red),
        Type::Water => Plain.fg(Blue),
        Type::Grass => Plain.fg(Green),
        Type::Electric => Plain.fg(Yellow),
        Type::Poison => Plain.fg(Magenta),
        // My console has a black background but in case there is
        // a white background specify a black background for white font
        _ => Plain.fg(White).bg(Black),
    }
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// Helper functions (you don't need to understand how they work yet)
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    ::std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
fn read_usize() -> usize {
    loop {
        match read_string().parse() {
            Ok(res) => return res,
            Err(_) => println!("That was not an integer! Please try again!"),
        }
    }
}