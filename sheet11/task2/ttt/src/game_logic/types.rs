use game_logic::Symbol;
use std::fmt::{Formatter, Display, Error};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerType {
    Human,
    Dumb,
    Smart
}

#[derive(Debug, Copy, Clone)]
pub struct Player {
    name: PlayerType,
    sym: Symbol
}

impl Player {
    // Returns an instance of a Player with the given PlayerType
    pub fn new(p: PlayerType, s: Symbol) -> Self {
        Player { name: p , sym: s}
    }

    pub fn name(&self) -> PlayerType {
        self.name
    }

    pub fn symbol(&self) -> Symbol {
        self.sym
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let content = match self.name {
            PlayerType::Human => "Human",
            PlayerType::Dumb => "Dumb AI",
            PlayerType::Smart => "Smart AI"
        };

        write!(f, "{}", content)
    }
}
