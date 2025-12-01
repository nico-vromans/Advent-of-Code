use super::*;
use aoc_core::Solver;

#[test]
fn test_solve_example() {
    let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let solver = Day01;
    let result = solver.solve(input);

    // Part 1 example answer is 3
    assert_eq!(result[0], "3");
    // Part 2 example answer is 6
    assert_eq!(result[1], "6");
}
