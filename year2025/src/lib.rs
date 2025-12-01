use aoc_core::Solver;

pub mod day001;

pub fn get_solver(day: u8) -> Option<Box<dyn Solver>> {
    match day {
        1 => Some(Box::new(day001::Day001)),
        _ => None,
    }
}
