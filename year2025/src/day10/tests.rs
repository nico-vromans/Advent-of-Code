use super::Day10;
use aoc_core::{Solver, read_input};

// Tests for Day 10 — based on the puzzle README and the provided input file.

#[test]
#[ignore = "Solver not implemented yet; unignore when solution is ready"]
fn test_day10_readme_example_part1() {
    // Example from README.txt
    let input: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}\n[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    let solver: Day10 = Day10;
    let result: Vec<String> = solver.solve(input);

    // README states the total fewest button presses across the three machines is 7
    assert_eq!(result[0], "7");
}

#[test]
fn test_day10_real_input_structure() {
    let input: String = read_input(2025, 10).expect("input file should exist for day 10");
    let solver: Day10 = Day10;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");
}
