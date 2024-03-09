use itertools::Itertools;
use std::f32::INFINITY;

fn get_float(p: Option<&str>) -> f32 {
    match str::parse::<f32>(p.unwrap().trim()) {
        Ok(res) => res,
        Err(_) => panic!("failed to parse float: \"{}\"", p.unwrap()),
    }
}

#[derive(Debug, Clone)]
struct Star {
    #[allow(dead_code)]
    name: String, // useful for debugging
    x: f32,
    y: f32,
    z: f32,
}

impl Star {
    fn new(line: &str) -> Self {
        let mut parts = line.split("   ").filter(|s| s.trim() != "");
        let name = parts.next().unwrap().to_string();
        parts.next(); // discard DIST

        let (x, y, z) = (
            get_float(parts.next()),
            get_float(parts.next()),
            get_float(parts.next()),
        );

        Star { name, x, y, z }
    }

    fn pythag(&self, other: &Self) -> f32 {
        f32::sqrt(
            f32::powi(self.x - other.x, 2)
                + f32::powi(self.y - other.y, 2)
                + f32::powi(self.z - other.z, 2),
        )
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();
    lines.next(); // discard header

    let closest = lines
        .map(|line| Star::new(line)) // parse stars
        .combinations(2) // get every pair of stars
        .map(|pair| pair[0].pythag(&pair[1])) // calculate the pythags
        .fold(INFINITY, |res, i| res.min(i)); // use the float-aware version of .min()

    println!("{:.3}", closest)
}
