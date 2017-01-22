extern crate clap;

use game_logic::Symbol;
use game_logic::types::{PlayerType, Player};
use self::clap::{App, Arg};

macro_rules! get_player {
    ($matcher:expr, $arg_name:expr) => {{
        match $matcher.value_of($arg_name) {
            Some("human") => PlayerType::Human,
            Some("dumb-ai") => PlayerType::Dumb,
            Some("smart-ai") => PlayerType::Smart,
            _ => unreachable!()
        }
    }};
}

/// Create clap-App and read command line arguments.
/// Returns both players.
pub fn parse_commands() -> (Player, Player) {
    let matches = App::new("ttt")
                    .version("1.0.0")
                    .about("Tic Tac Toe")
                    .author(concat!("Franziska Becker <buecher.apps@gmail.com>,",
                        " René Wanrking <rwarnking@gmail.com>"))
                    .arg(Arg::with_name("player_one")
                        .help("Selects player one.")
                        .possible_values(&["human", "dumb-ai", "smart-ai"])
                        .required(true))
                    .arg(Arg::with_name("player_two")
                        .possible_values(&["human", "dumb-ai", "smart-ai"])
                        .help("Selects player two.")
                        .required(true))
                    .get_matches();

    (Player::new(get_player!(matches, "player_one"), Symbol::Cross),
        Player::new(get_player!(matches, "player_two"), Symbol::Circle))
}

/// Reads the input position from the human player.
pub fn read_position_input() -> Option<(char, u8)> {
    let mut buffer = String::new();
    ::std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Shorten string to length of two
    if buffer.len() >= 2 {
        buffer.truncate(2);

        if let Some(last) = buffer.pop() {
            if let Some(number) = last.to_digit(10) {
                return Some((buffer.remove(0), number as u8))
            }
        }
    }

    None
}
