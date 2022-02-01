use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

use rubiks::{Color, Cube, Piece, Turn, Valid};

#[derive(Debug, Clone)]
pub struct Cube3x3 {
    state: [Piece; 27],
}

impl Cube3x3 {
    pub fn new() -> Cube3x3 {
        Cube3x3 {
            state: Self::get_solved_state(),
        }
    }

    const fn get_solved_state() -> [Piece; 27] {
        use Color::*;
        [
            Piece::Corner(Red, Yellow, Blue),
            Piece::Edge(Yellow, Blue),
            Piece::Corner(Blue, Yellow, Orange),
            Piece::Edge(Yellow, Red),
            Piece::Center(Yellow),
            Piece::Edge(Yellow, Orange),
            Piece::Corner(Green, Yellow, Red),
            Piece::Edge(Yellow, Green),
            Piece::Corner(Orange, Yellow, Green),
            Piece::Edge(Red, Blue),
            Piece::Center(Blue),
            Piece::Edge(Blue, Orange),
            Piece::Center(Red),
            Piece::Inner,
            Piece::Center(Orange),
            Piece::Edge(Green, Red),
            Piece::Center(Green),
            Piece::Edge(Orange, Green),
            Piece::Corner(Red, White, Blue),
            Piece::Edge(White, Blue),
            Piece::Corner(Blue, White, Orange),
            Piece::Edge(White, Red),
            Piece::Center(White),
            Piece::Edge(White, Orange),
            Piece::Corner(Green, White, Red),
            Piece::Edge(White, Green),
            Piece::Corner(Orange, White, Green),
        ]
    }
}

impl PartialEq for Cube3x3 {
    fn eq(&self, other: &Cube3x3) -> bool {
        self.state == other.state
    }
}

impl Eq for Cube3x3 {}

impl Hash for Cube3x3 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl Cube for Cube3x3 {
    fn get_turns(&self) -> Vec<Turn> {
        Turn::get_all_3x3().to_vec()
    }

    fn turn(&mut self, turn: Turn) -> Valid {
        for t in turn.break_() {
            match &t {
                Turn::R => todo!(),
                Turn::L => todo!(),
                Turn::F => todo!(),
                Turn::B => todo!(),
                Turn::U => {
                    let tmp = self.state[0..9].to_vec();
                    self.state[2] = tmp[0];
                    self.state[5] = tmp[1];
                    self.state[8] = tmp[2];
                    self.state[1] = tmp[3];
                    self.state[7] = tmp[5];
                    self.state[0] = tmp[6];
                    self.state[3] = tmp[7];
                    self.state[6] = tmp[8];
                }
                Turn::D => {
                    let tmp = self.state[18..27].to_vec();
                    self.state[24] = tmp[8];
                    self.state[21] = tmp[7];
                    self.state[18] = tmp[6];
                    self.state[25] = tmp[5];
                    self.state[19] = tmp[3];
                    self.state[26] = tmp[2];
                    self.state[23] = tmp[1];
                    self.state[20] = tmp[0];
                }
                _ => return Valid::Invalid,
            }
        }
        Valid::Valid
    }

    fn is_solved(&self) -> bool {
        self.state == Self::get_solved_state()
    }
}
