use aoc_core::Solver;

pub struct Day01;

impl Solver for Day01 {
    fn solve(&self, input: &str) -> Vec<String> {
        fn parse_instructions(input: &str) -> impl Iterator<Item = (char, i32)> + '_ {
            input.lines().filter_map(|line| {
                let line: &str = line.trim();

                if line.is_empty() {
                    return None;
                }

                let direction: char = line.chars().next()?.to_ascii_uppercase();
                let amount: i32 = line[1..].parse().ok()?;

                Some((direction, amount))
            })
        }
        fn part1(input: &str) -> i32 {
            let start_position: i32 = 50;
            let mut current_position: i32 = start_position;
            let mut stop_at_zero: i32 = 0;

            for (direction, amount) in parse_instructions(input) {
                match direction {
                    // Use rem_euclid to handle negative numbers correctly for circular wrapping
                    'L' => current_position = (current_position - amount).rem_euclid(100),
                    'R' => current_position = (current_position + amount).rem_euclid(100),
                    _ => panic!("Invalid direction: {}", direction),
                }

                if current_position == 0 {
                    stop_at_zero += 1;
                }
            }

            stop_at_zero
        }

        fn part2(input: &str) -> i32 {
            let mut current_position: i32 = 50;
            let mut total_zeros: i32 = 0;

            for (direction, amount) in parse_instructions(input) {
                for _ in 0..amount {
                    match direction {
                        'L' => current_position = (current_position - 1).rem_euclid(100),
                        'R' => current_position = (current_position + 1).rem_euclid(100),
                        _ => panic!("Unknown direction: {}", direction),
                    }

                    if current_position == 0 {
                        total_zeros += 1;
                    }
                }
            }

            total_zeros
        }

        vec![part1(input).to_string(), part2(input).to_string()]
    }
}
