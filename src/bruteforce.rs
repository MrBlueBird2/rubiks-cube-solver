use std::{
    cmp::{Eq, PartialEq},
    collections::{HashSet, VecDeque},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use rubiks::{Cube, CubeSolver, Turn, Valid};

struct MoveWrapper<T: Cube> {
    cube: T,
    turns: Vec<Turn>,
}

impl<T: Cube> MoveWrapper<T> {
    fn new(cube: T) -> MoveWrapper<T> {
        MoveWrapper {
            cube,
            turns: vec![],
        }
    }

    fn turn(&mut self, turn: Turn) -> Valid {
        self.turns.push(turn);
        self.cube.turn(turn)
    }

    fn get_turns(&self) -> Vec<Turn> {
        let mut moves = self.cube.get_turns();
        if let Some(t) = self.turns.last() {
            match t {
                Turn::U | Turn::U2 | Turn::U3 => moves.drain(0..3),
                Turn::R | Turn::R2 | Turn::R3 => moves.drain(3..6),
                Turn::L | Turn::L2 | Turn::L3 => moves.drain(6..9),
                Turn::F | Turn::F2 | Turn::F3 => moves.drain(9..12),
                Turn::D | Turn::D2 | Turn::D3 => moves.drain(12..15),
                Turn::B | Turn::B2 | Turn::B3 => moves.drain(15..18),
            };
        }
        moves
    }

    fn is_solved(&self) -> bool {
        self.cube.is_solved()
    }
}

impl<T: Cube + Clone> Clone for MoveWrapper<T> {
    fn clone(&self) -> MoveWrapper<T> {
        MoveWrapper {
            cube: self.cube.clone(),
            turns: self.turns.clone(),
        }
    }
}

impl<T: Cube + Hash> Hash for MoveWrapper<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cube.hash(state);
    }
}

impl<T: Cube + PartialEq> PartialEq for MoveWrapper<T> {
    fn eq(&self, other: &MoveWrapper<T>) -> bool {
        self.cube == other.cube
    }
}

impl<T: Cube + Eq> Eq for MoveWrapper<T> {}

pub struct BruteForceSolver<T: Cube>(PhantomData<T>);

impl<T: Cube> BruteForceSolver<T> {
    pub fn new() -> BruteForceSolver<T> {
        BruteForceSolver(PhantomData)
    }
}

impl<T: Cube + Hash + Eq + Clone> CubeSolver for BruteForceSolver<T> {
    type Cube = T;

    fn solve(&self, cube: T) -> Vec<Turn> {
        let cube = MoveWrapper::new(cube);
        let mut visited = HashSet::new();
        if cube.is_solved() {
            return cube.turns;
        }
        let mut queue = VecDeque::new();
        queue.push_back(cube);
        while let Some(c) = queue.pop_front() {
            for t in c.get_turns() {
                let mut new_cube = c.clone();
                new_cube.turn(t).unwrap();
                if c.is_solved() {
                    return c.turns;
                }
                if !visited.contains(&new_cube) {
                    visited.insert(new_cube.clone());
                    queue.push_back(new_cube);
                }
            }
        }
        unreachable!();
    }
}
