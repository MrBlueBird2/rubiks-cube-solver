use rubiks::{Cube, CubeSolver, Turn};
mod bruteforce;
mod cube3x3;

use crate::bruteforce::BruteForceSolver;
use crate::cube3x3::Cube3x3;

fn main() {
    let mut cube = Cube3x3::new();
    let solver = BruteForceSolver::new();
    cube.turn(Turn::U2).unwrap();
    let sol = solver.solve(cube);
    println!("{:?}", sol);
}
