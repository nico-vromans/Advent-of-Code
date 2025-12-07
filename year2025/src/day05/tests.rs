use super::Day05;
use aoc_core::{Solver, read_input};

#[test]
fn test_day05_readme_example_part1() {
    // Example from README.md
    let input: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    let solver: Day05 = Day05;
    let result: Vec<String> = solver.solve(input);

    // Part 1: three of the IDs are fresh
    assert_eq!(result[0], "3");
}

#[test]
fn test_day05_readme_example_part2() {
    // Same README example; Part 2 counts total IDs covered by the union of ranges.
    let input: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    let solver: Day05 = Day05;
    let result: Vec<String> = solver.solve(input);

    // Part 2: union is {3..=5} plus {10..=20} => 3 + 11 = 14
    assert_eq!(result[1], "14");
}

#[test]
fn test_day05_real_input_known_answer_part1() {
    let input: String = read_input(2025, 5).expect("input file should exist for day 05");
    let solver: Day05 = Day05;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct Part 1 answer (computed via runner)
    assert_eq!(result[0], "674");
}

#[test]
fn test_day05_real_input_known_answer_part2() {
    let input: String = read_input(2025, 5).expect("input file should exist for day 05");
    let solver: Day05 = Day05;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct Part 2 answer (computed via runner)
    assert_eq!(result[1], "352509891817881");
}
