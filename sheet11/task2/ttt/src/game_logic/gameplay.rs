use game_logic::{Board, Position, OFFSET};
use game_logic::types::{Player, PlayerType};
use game_logic::ai::get_ai_move;
use interface::read_position_input;

/// Gameplay loop.
pub fn play(one: Player, two: Player) {
    let mut turn = true;
    let mut board = Board::new();

    loop {
        println!("{}", board);

        //
        if board.player_won() {
            if turn {
                println!("Player 2 ({}) won!", two);
            } else {
                println!("Player 1 ({}) won!", one);
            }
            break;
        } else if board.is_full() {
            println!("Unfortunately, no one won! =P");
            break;
        }

        if turn {
            execute_round(&mut board, one);
        } else {
            execute_round(&mut board, two);
        }
        turn = !turn;
    }
}

/// Executes a round by getting the move of the current player
fn execute_round(board: &mut Board, player: Player) {
    println!("{:?} player turn:", player.name());

    let player_move = match player.name() {
        PlayerType::Human => get_human_move(&board),
        _ => get_ai_move(&board, player)
    };

    board.mark(player.symbol(), player_move);
}

/// Reads and returns the move for the human player
pub fn get_human_move(board: &Board) -> Position {
    loop {
        match read_position_input() {
            Some(pos) => {
                if !Position::is_pos(pos.0, pos.1) {
                    println!("Please enter a position on the board!");
                    println!("\"{:?}\" is not a valid position", pos);
                } else if !board.is_empty(Position::pos(pos.0 as u8 - OFFSET, pos.1)) {
                    println!("Please enter a position on the board that is still empty!");
                }
                else {
                    return Position::pos(pos.0 as u8 - OFFSET, pos.1)
                }
            },
            None => {
                println!("Please enter a valid position (a-c1-3) on the board!");
            }
        }
    }
}
