use std::collections::HashMap;

fn main() {
    let mut blocks = include_str!("input.txt").split("\n\n");
    let (grid, routes) = match (blocks.next(), blocks.next()) {
        (Some(top), Some(bottom)) => (top, bottom),
        _ => panic!("Expected at least two parts"),
    };

    let mut lines = grid.lines();
    let cols: Vec<&str> = lines.next().unwrap().split_ascii_whitespace().collect();

    let mut distances: HashMap<String, i32> = HashMap::new();

    for line in lines {
        let mut parts = line.split_ascii_whitespace();
        let from = parts.next().unwrap();
        for (i, dist) in parts.enumerate() {
            distances.insert(format!("{from}-{}", cols[i]), str::parse(dist).unwrap());
        }
    }
    // println!("{:#?}", distances);

    let total: i32 = routes
        .lines()
        .map(|route| {
            let (_, steps) = route.split_once(": ").unwrap();
            let steps = steps.split(" -> ");

            steps
                .clone()
                .zip(steps.skip(1))
                .map(|(l, r)| distances.get(&format!("{l}-{r}")).unwrap())
                .sum::<i32>()

            // println!("{total}")
        })
        .sum();
    println!("{total}")
}
