use aoc_core::Solver;

pub mod day01;
pub mod day02;

pub fn get_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day01::Day01)),
        2 => Some(Box::new(day02::Day02)),
        _ => None,
    }
}
