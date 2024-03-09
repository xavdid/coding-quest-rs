use std::collections::HashMap;

fn main() {
    let mut costs: HashMap<&str, i32> = HashMap::new();

    for line in include_str!("input.txt").lines() {
        let mut parts = line.split_ascii_whitespace();

        let name = parts.next().unwrap().strip_suffix(":").unwrap();

        let line_item = parts.next().unwrap();
        let multiplier = match line_item {
            "Seat" | "Luggage" | "Tax" | "Fee" | "Meals" => 1,
            "Discount" | "Rebate" => -1,
            _ => panic!("unrecognized: {line_item}"),
        };

        let value = str::parse::<i32>(parts.next().unwrap()).unwrap() * multiplier;

        // costs.insert(name, costs.get(name).or(Some(&0)).unwrap() + value);
        costs
            .entry(name)
            .and_modify(|v| *v += value)
            .or_insert(value);

        // costs.entry(name).or_insert(0).
    }

    // println!("{:?}", costs);

    let result = costs
        .iter()
        .min_by(|(_, v), (_, vv)| v.partial_cmp(vv).unwrap());
    println!("cheapest: {}", result.unwrap().1);
}
