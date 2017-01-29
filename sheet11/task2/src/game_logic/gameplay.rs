use game_logic::Board;
use game_logic::types::Player;

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
    println!("{} player turn:", player);
    let player_move = player.get_move(board);
    board.mark(player.symbol(), player_move);
}
