// Michael Williamson
// Sudoku Puzzle Solver

use sudokugen::{Puzzle, BoardSize};

fn main() {
    let puzzle = Puzzle::generate(BoardSize::NineByNine);
    println!("Puzzle\n{}", puzzle.board());

    let temp: String = puzzle.board().to_string();

    println!("{}", temp);

    println!("Solution\n{}", puzzle.solution());
}
