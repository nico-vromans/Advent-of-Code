#[cfg(test)]
mod tests;

pub trait Solver {
    fn solve(&self, input: &str) -> Vec<String>;
}

pub struct Day {
    pub name: &'static str,
    pub solver: Box<dyn Solver>,
}

pub fn read_input(year: u16, day: u8) -> Result<String, std::io::Error> {
    let base_paths: [String; 3] = [
        format!("year{}/src/day{:02}/input", year, day), // From workspace root
        format!("src/day{:02}/input", day),              // From year crate root
        format!("day{:02}/input", day),                  // From src dir (unlikely but possible)
    ];

    let extensions: [&str; 2] = ["txt", "in"];

    for base in &base_paths {
        for ext in &extensions {
            let path: String = format!("{base}.{ext}");
            if let Ok(content) = std::fs::read_to_string(path) {
                return Ok(content);
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "Could not find input file. Checked: {:?}",
            base_paths
                .iter()
                .flat_map(|b: &String| extensions.iter().map(move |e: &&str| format!("{b}.{e}")))
                .collect::<Vec<_>>()
        ),
    ))
}
