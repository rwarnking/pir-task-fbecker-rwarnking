use game_logic::{Symbol, Board, Position};
use std::fmt::{Formatter, Display, Error};
use game_logic::ai::get_ai_move;
use interface::get_human_move;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    Dumb,
    Smart
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    ptype: PlayerType,
    sym: Symbol
}

impl Player {
    // Returns an instance of a Player with the given PlayerType
    pub fn new(p: PlayerType, s: Symbol) -> Self {
        Player { ptype: p , sym: s}
    }

    pub fn ptype(&self) -> PlayerType {
        self.ptype
    }

    pub fn symbol(&self) -> Symbol {
        self.sym
    }

    pub fn get_move(&self, board: &Board) -> Position {
        match self.ptype {
            PlayerType::Human => get_human_move(board),
            _ => get_ai_move(board, *self)
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let content = match self.ptype {
            PlayerType::Human => "Human",
            PlayerType::Dumb => "Dumb AI",
            PlayerType::Smart => "Smart AI"
        };

        write!(f, "{}", content)
    }
}
