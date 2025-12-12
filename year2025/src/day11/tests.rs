use super::Day11;
use aoc_core::{Solver, read_input};

// Tests for Day 11 — based on the puzzle README and the provided input file.

#[test]
#[ignore = "Solver not implemented yet; unignore when solution is ready"]
fn test_day11_readme_example_part1() {
    // Example from README.txt
    let input: &str = "aaa: you hhh\nyou: bbb ccc\nbbb: ddd eee\nccc: ddd eee fff\nddd: ggg\neee: out\nfff: out\nggg: out\nhhh: ccc fff iii\niii: out";

    let solver: Day11 = Day11;
    let result: Vec<String> = solver.solve(input);

    // README states there are 5 different paths from you to out
    assert_eq!(result[0], "5");
}

#[test]
fn test_day11_real_input_structure() {
    let input: String = read_input(2025, 11).expect("input file should exist for day 11");
    let solver: Day11 = Day11;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");
}
