use super::Day09;
use aoc_core::{Solver, read_input};

// Tests for Day 9 — based on the puzzle README and the provided input file.

#[test]
fn test_day09_readme_example_part1() {
    // Example from README.md
    let input: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3";

    let solver: Day09 = Day09;
    let result: Vec<String> = solver.solve(input);

    // README states the largest rectangle area is 50
    assert_eq!(result[0], "50");
}

#[test]
fn test_day09_real_input_known_answer_part1() {
    let input: String = read_input(2025, 9).expect("input file should exist for day 09");
    let solver: Day09 = Day09;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct Part 1 answer (computed via runner)
    assert_eq!(result[0], "4759930955");
}
