use super::Day08;
use aoc_core::{Solver, read_input};

#[test]
fn test_day08_readme_example_part1() {
    // Example from README.md (20 junction box coordinates)
    let input: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

    let solver: Day08 = Day08;
    let result: Vec<String> = solver.solve(input);

    // README: after 10 shortest connections, product of sizes of three largest circuits is 40
    assert_eq!(result[0], "40");
}

#[test]
fn test_day08_real_input_known_answer_part1() {
    let input: String = read_input(2025, 8).expect("input file should exist for day 08");
    let solver: Day08 = Day08;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct Part 1 answer (computed via runner)
    assert_eq!(result[0], "79056");
}

#[test]
fn test_day08_readme_example_part2() {
    // Example from README.md (same 20 junction box coordinates)
    let input: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

    let solver: Day08 = Day08;
    let result: Vec<String> = solver.solve(input);

    // README: last connection to unify all boxes is between X=216 and X=117 -> 216*117 = 25272
    assert_eq!(result[1], "25272");
}

#[test]
fn test_day08_real_input_known_answer_part2() {
    let input: String = read_input(2025, 8).expect("input file should exist for day 08");
    let solver: Day08 = Day08;
    let result: Vec<String> = solver.solve(&input);

    // Ensure two parts are returned
    assert_eq!(result.len(), 2, "solver should return two answers");

    // Known correct Part 2 answer (computed via runner)
    assert_eq!(result[1], "4639477");
}
