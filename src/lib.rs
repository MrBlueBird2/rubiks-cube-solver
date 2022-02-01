use std::hash::Hash;

pub trait Cube {
    fn get_turns(&self) -> Vec<Turn>;
    fn turn(&mut self, turn: Turn) -> Valid;
    fn is_solved(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[must_use]
pub enum Valid {
    Valid,
    Invalid,
}

impl Valid {
    pub fn is_valid(&self) -> bool {
        match self {
            Valid::Valid => true,
            Valid::Invalid => false,
        }
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    pub fn unwrap(self) {
        if self.is_invalid() {
            panic!("Invalid Turn");
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Color {
    Red,
    White,
    Yellow,
    Blue,
    Green,
    Orange,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Turn {
    U,
    U2,
    U3,
    R,
    R2,
    R3,
    L,
    L2,
    L3,
    F,
    F2,
    F3,
    D,
    D2,
    D3,
    B,
    B2,
    B3,
}

impl Turn {
    pub fn break_(&self) -> Vec<Turn> {
        match self {
            Turn::U => vec![Turn::U],
            Turn::U2 => vec![Turn::U, Turn::U],
            Turn::U3 => vec![Turn::U, Turn::U, Turn::U],
            Turn::R => vec![Turn::R],
            Turn::R2 => vec![Turn::R, Turn::R],
            Turn::R3 => vec![Turn::R, Turn::R, Turn::R],
            Turn::L => vec![Turn::L],
            Turn::L2 => vec![Turn::L, Turn::L],
            Turn::L3 => vec![Turn::L, Turn::L, Turn::L],
            Turn::F => vec![Turn::F],
            Turn::F2 => vec![Turn::F, Turn::F],
            Turn::F3 => vec![Turn::F, Turn::F, Turn::F],
            Turn::D => vec![Turn::D],
            Turn::D2 => vec![Turn::D, Turn::D],
            Turn::D3 => vec![Turn::D, Turn::D, Turn::D],
            Turn::B => vec![Turn::B],
            Turn::B2 => vec![Turn::B, Turn::B],
            Turn::B3 => vec![Turn::B, Turn::B, Turn::B],
        }
    }

    pub const fn get_all_3x3() -> [Turn; 18] {
        [
            Turn::U,
            Turn::U2,
            Turn::U3,
            Turn::R,
            Turn::R2,
            Turn::R3,
            Turn::L,
            Turn::L2,
            Turn::L3,
            Turn::F,
            Turn::F2,
            Turn::F3,
            Turn::D,
            Turn::D2,
            Turn::D3,
            Turn::B,
            Turn::B2,
            Turn::B3,
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Piece {
    Inner,
    Center(Color),
    Edge(Color, Color),
    Corner(Color, Color, Color),
}

pub trait CubeSolver {
    type Cube;
    fn solve(&self, cube: Self::Cube) -> Vec<Turn>;
}
