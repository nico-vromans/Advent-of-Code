use super::*;
use aoc_core::{read_input, Solver};

// Tests for Day 2 — based on the puzzle README and the provided input file.

#[test]
fn test_solve_example_single() {
    // The README example (wrapped there for legibility) as a single line of ranges.
    // It should produce a sum of 33 for Part 1.
    let input: &str = "11-22";

    let solver = Day02;
    let result: Vec<String> = solver.solve(input);

    // Part 1 example answer from README
    assert_eq!(result[0], "33");

    // Part 2 has not been specified in the README excerpt; once defined, add/assert here.
}

#[test]
fn test_solve_example_full() {
    // The README example (wrapped there for legibility) as a single line of ranges.
    // It should produce a sum of 1227775554 for Part 1.
    let input: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
824824821-824824827,2121212118-2121212124";

    let solver = Day02;
    let result: Vec<String> = solver.solve(input);

    // Part 1 example answer from README
    assert_eq!(result[0], "1227775554");

    // Part 2 has not been specified in the README excerpt; once defined, add/assert here.
}

#[test]
fn test_real_input_smoke() {
    // This is a smoke test for the real input file to help quickly verify integration.
    // Once you have the correct answers, replace the asserts with the known values and
    // remove the #[ignore] to run it in CI/local runs.
    let input: String = read_input(2025, 2).expect("input file should exist for day 02");
    let solver = Day02;
    let result: Vec<String> = solver.solve(&input);

    // Ensure we always return two parts
    assert_eq!(result.len(), 2, "solver should return two answers");

    // TODO: Replace these with known answers when available and un-ignore the test.
    assert_eq!(result[0], "53420042388");
    // assert_eq!(result[1], "<part2>");
}
