use game_logic::{Board, Symbol, WIN_CONDITIONS, Position};
use game_logic::types::{Player, PlayerType};
use game_logic::Position::*;
use game_logic::Symbol::*;

/// Returns a dumb move that just looks for an empty field
/// in a line that no other marks.
fn get_dumb_move(board: &Board) -> Position {
    for outer in 1..4 {
        for inner in 1..4 {
            let pos = Position::pos(outer, inner);
            if board.is_empty(pos) {
                return pos
            }
        }
    }
    unreachable!()
}

/// Returns a move for the AI, depending on which AI it is.
pub fn get_ai_move(board: &Board, p: Player) -> Position {
    // if ai is stupid
    if p.ptype() == PlayerType::Dumb {
        get_dumb_move(board)
    // if ai is smart
    } else {
        // store opponent symbol
        let opponent = match p.symbol() {
            Cross => Circle,
            _ => Cross,
        };

        let mut prevent = None;
        for &line in WIN_CONDITIONS {
            // check if a winning move can be made
            if let Some(next_move) = check_win_condition(board, line, p.symbol()) {
                return next_move
            }
            // check if opponent can win but only store it, so a winning move
            // is preferred if present
            if let Some(s) = check_win_condition(board, line, opponent) {
                prevent = Some(s);
            }
        }
        // if the opponent can win in the next round prevent that
        if prevent.is_some() {
            return prevent.unwrap()
        }

        // else look for best position to mark
        let map = write_values(board, p.symbol());
        if let Some(&(index, _)) = map.iter().max_by_key(|elem| elem.1) {
            // gets the 'best' move possible
            return *WIN_CONDITIONS[index].iter()
                                         .filter(|elem| board.is_empty(**elem))
                                         .max_by_key(|elem| occurences_value(**elem))
                                         .unwrap()
        }
        // otherwise just return a dumb move
        get_dumb_move(board)
    }
}

/// Calculate values for each position on the field.
fn write_values(board: &Board, symbol: Symbol) -> Vec<(usize, i32)> {
    let mut map = Vec::new();

    for index in 0..8 {
        let mut current_value = 0;
        for pos in WIN_CONDITIONS[index] {
            // adjust value of a field accordind to its symbol
            current_value += symbol_value(board.get_symbol(*pos), symbol);
            if board.is_empty(*pos) {
                current_value += occurences_value(*pos);
            }
        }
        map.push((index, current_value));
    }
    map
}

/// Returns a value for a field depending on its Symbol.
fn symbol_value(s: Symbol, player: Symbol) -> i32 {
    match s {
        Empty => 0,
        _ => {
            if s == player {
                1
            } else {
                -5
            }
        }
    }
}

/// Returns a value for a field depending on how many winning
/// moves it occurs in.
fn occurences_value(pos: Position) -> i32 {
    match pos {
        TopLeft | TopRight | BottomLeft | BottomRight => 3,
        MiddleCenter => 4,
        _ => 2
    }

}

/// Checks whether the player with the Symbol in s can make a winning move.
fn check_win_condition(board: &Board, line: &[Position], s: Symbol) -> Option<Position> {
    match (board.get_symbol(line[0]), board.get_symbol(line[1]), board.get_symbol(line[2])) {
        (Empty, a, b) if a == b && a == s => Some(line[0]),
        (a, Empty, b) if a == b && a == s => Some(line[1]),
        (a, b, Empty) if a == b && a == s => Some(line[2]),
        _ => None
    }
}
