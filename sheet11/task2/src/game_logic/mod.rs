pub mod gameplay;
pub mod types;
pub mod ai;

use std::fmt::{Formatter, Display, Error};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use self::Position::*;

pub const WIN_CONDITIONS: &'static [&'static [Position]] = &[
        // horizontal
        &[TopLeft, TopCenter, TopRight],
        &[MiddleLeft, MiddleCenter, MiddleRight],
        &[BottomLeft, BottomCenter, BottomRight],
        // vertical
        &[TopLeft, MiddleLeft, BottomLeft],
        &[TopCenter, MiddleCenter, BottomCenter],
        &[TopRight, MiddleRight, BottomRight],
        // diagonal
        &[TopLeft, MiddleCenter, BottomRight],
        &[TopRight, MiddleCenter, BottomLeft]];

pub const OFFSET: u8 = 96;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight
}

impl Position {

    pub fn is_pos(a: char, b: u8) -> bool {
        match (a, b) {
            ('a' ... 'c', 1 ... 3) => true,
            _ => false
        }
    }

    fn pos(a: u8, b: u8) -> Self {
        match (a, b) {
            (1, 1) => TopLeft,
            (1, 2) => TopCenter,
            (1, 3) => TopRight,
            (2, 1) => MiddleLeft,
            (2, 2) => MiddleCenter,
            (2, 3) => MiddleRight,
            (3, 1) => BottomLeft,
            (3, 2) => BottomCenter,
            (3, 3) => BottomRight,
            _ => unreachable!()
        }
    }

    fn value(&self) -> (u8, u8) {
        match *self {
            TopLeft => (1, 1),
            TopCenter => (1, 2),
            TopRight => (1, 3),
            MiddleLeft => (2, 1),
            MiddleCenter => (2, 2),
            MiddleRight => (2, 3),
            BottomLeft => (3, 1),
            BottomCenter => (3, 2),
            BottomRight => (3, 3)
        }
    }
}

impl ::std::cmp::Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().0.hash(state);
        self.value().1.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Symbol {
    Circle,
    Cross,
    Empty
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let symbol = match *self{
            Symbol::Circle => "o",
            Symbol::Cross => "x",
            Symbol::Empty => " "
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug)]
pub struct Board {
    fields: HashMap<Position, Symbol>
}

impl Board {
    // return new board where every field is empty
    pub fn new() -> Self {
        let mut h = HashMap::with_capacity(9);
        for outer in 1..4 {
            for inner in 1..4 {
                h.insert(Position::pos(outer, inner), Symbol::Empty);
            }
        }
        Board { fields: h }
    }

    // get the symbol of the board at position 'pos'
    pub fn get_symbol(&self, pos: Position) -> Symbol {
        *self.fields.get(&pos).unwrap()
    }

    // mark a field on the board
    pub fn mark(&mut self, s: Symbol, pos: Position) {
        self.fields.insert(pos, s);
    }

    // return whether field at pos is empty or not
    pub fn is_empty(&self, pos: Position) -> bool {
        self.fields.get(&pos) == Some(&Symbol::Empty)
    }

    // return wether one player marked a line of three fields
    pub fn player_won(&self) -> bool {
        WIN_CONDITIONS.iter()
                      .any(|&line| self.fields.get(&line[0]).unwrap() != &Symbol::Empty &&
                                   self.fields.get(&line[0]).unwrap() ==
                                   self.fields.get(&line[1]).unwrap() &&
                                   self.fields.get(&line[1]) ==
                                   self.fields.get(&line[2]))
    }

    // return whether all fields are marked
    pub fn is_full(&self) -> bool {
        self.fields.iter()
                   .all(|elem| elem.1 != &Symbol::Empty)
    }
}

#[allow(unused_must_use)]
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "   1  2  3\n");
        write!(f, "--|--|--|--|\n");
        for outer in 1..4 {
            let c = match outer {
                1 => 'a',
                2 => 'b',
                3 => 'c',
                _ => unreachable!()
            };
            write!(f, "{} |{} |{} |{} |\n", c,
                self.fields.get(&Position::pos(outer, 1)).unwrap(),
                self.fields.get(&Position::pos(outer, 2)).unwrap(),
                self.fields.get(&Position::pos(outer, 3)).unwrap());
        }
        write!(f, "--|--|--|--|\n")
    }
}
