pub trait Solver {
    fn solve(&self, input: &str) -> Vec<String>;
}

pub struct Day {
    pub name: &'static str,
    pub solver: Box<dyn Solver>,
}

pub fn read_input(year: u16, day: u8) -> Result<String, std::io::Error> {
    let paths: [String; 3] = [
        format!("year{}/src/day{:02}/input.txt", year, day), // From workspace root
        format!("src/day{:02}/input.txt", day),              // From year crate root
        format!("day{:02}/input.txt", day),                  // From src dir (unlikely but possible)
    ];

    for path in &paths {
        if let Ok(content) = std::fs::read_to_string(path) {
            return Ok(content);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!("Could not find input file. Checked: {:?}", paths),
    ))
}
