use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Tile {
    Floor,
    Elevator,
}

type GridPoint = (usize, usize, usize);
type Grid = HashMap<GridPoint, Location>;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Location {
    tile: Tile,
    is_exit: bool,
    row: usize,
    col: usize,
    floor: usize,
}

impl Location {
    fn neighbors(&self) -> Vec<GridPoint> {
        let mut result: Vec<GridPoint> = Vec::new();

        let floors = if self.tile == Tile::Elevator {
            vec![0, 1]
        } else {
            vec![self.floor]
        };

        for floor in floors {
            // going negative is bad because these are unsigned, but going off the positive edge is fine (it won't be in the grid)

            // UP
            if self.row > 0 {
                result.push((self.row - 1, self.col, floor));
            }

            // LEFT
            if self.col > 0 {
                result.push((self.row, self.col - 1, floor));
            }

            // DOWN
            result.push((self.row + 1, self.col, floor));
            // RIGHT
            result.push((self.row, self.col + 1, floor));
        }

        result
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    cost: u32,
    location: Location,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_block(grid: &mut Grid, floor: usize, block: &str) {
    let grid_height = block.lines().count();
    let grid_width = block.lines().nth(1).unwrap().len();
    let exit = (grid_height - 2, grid_width - 1, 0);

    for (row, line) in block.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let loc = (row, col, floor);
            match c {
                '.' | '$' => {
                    grid.insert(
                        loc,
                        Location {
                            row,
                            col,
                            floor,
                            tile: if c == '.' {
                                Tile::Floor
                            } else {
                                Tile::Elevator
                            },
                            is_exit: loc == exit,
                        },
                    );
                }
                _ => {}
            }
        }
    }
}

fn explore_maze(input: &str) -> u32 {
    let start = State {
        cost: 0,
        location: Location {
            row: 1,
            col: 0,
            floor: 0,
            is_exit: false,
            tile: Tile::Floor,
        },
    };

    let mut grid: Grid = HashMap::new();

    for (floor, block) in input.split("\n\n").enumerate() {
        parse_block(&mut grid, floor, block);
    }

    // let mut queue = BinaryHeap::from([start]);
    let mut queue = BinaryHeap::new();
    queue.push(start);

    let mut visited: HashSet<Location> = HashSet::new();

    while !queue.is_empty() {
        let current = queue.pop().unwrap();

        if visited.contains(&current.location) {
            continue;
        }
        visited.insert(current.location.clone());

        if current.location.is_exit {
            return current.cost;
        }

        // add all neighbors to queue
        for potential_neighbor in current.location.neighbors() {
            if let Some(n) = grid.get(&potential_neighbor) {
                if !visited.contains(n) {
                    queue.push(State {
                        cost: current.cost + 1,
                        location: n.clone(),
                    });
                }
            }
        }
    }

    panic!("--> never found exit!")
}

fn main() {
    let result = explore_maze(include_str!("input.txt"));

    assert_eq!(result, 250);

    println!("steps: {result}");
}
