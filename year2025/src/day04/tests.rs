use super::Day04;
use aoc_core::{Solver, read_input};

// Tests for Day 4 — based on the puzzle README and the provided input file.

#[test]
fn test_day04_readme_example_part1() {
    // Example from README.md
    let input: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    let solver: Day04 = Day04;
    let result: Vec<String> = solver.solve(input);

    // README states there are 13 accessible rolls for Part 1
    assert_eq!(result[0], "13");
}

#[test]
fn test_day04_readme_example_part2() {
    // Example from README.md
    let input: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    let solver: Day04 = Day04;
    let result: Vec<String> = solver.solve(input);

    // README states the process can remove a total of 43 rolls for Part 2
    assert_eq!(result[1], "43");
}

#[test]
fn test_day04_real_input_known_answer_part1() {
    let input: String = read_input(2025, 4).expect("input file should exist for day 04");
    let solver: Day04 = Day04;
    let result: Vec<String> = solver.solve(&input);

    // Ensure we always return two parts
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct answer for Part 1 (computed once and locked in)
    assert_eq!(result[0], "1370");
}

#[test]
fn test_day04_real_input_known_answer_part2() {
    let input: String = read_input(2025, 4).expect("input file should exist for day 04");
    let solver: Day04 = Day04;
    let result: Vec<String> = solver.solve(&input);

    // Ensure we always return two parts
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct answer for Part 2 (computed via runner)
    assert_eq!(result[1], "8437");
}
