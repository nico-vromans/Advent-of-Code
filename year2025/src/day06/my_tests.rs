use super::Day06;
use aoc_core::Solver;

#[test]
fn test_part1_simple() {
    let input = "1 2\n3 4\n+ *";
    let solver = Day06;
    let result = solver.solve(input);
    assert_eq!(result[0], "12");
}

#[test]
fn test_part1_readme_example_adapted() {
    let input = "123 328 51 64\n45 64 387 23\n6 98 215 314\n* + * +";
    let solver = Day06;
    let result = solver.solve(input);
    assert_eq!(result[0], "4277556");
}
