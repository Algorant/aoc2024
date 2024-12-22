use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_cost(&self, other: &Direction) -> i32 {
        if self == other {
            0
        } else if (self == &Direction::North && other == &Direction::South)
            || (self == &Direction::South && other == &Direction::North)
            || (self == &Direction::East && other == &Direction::West)
            || (self == &Direction::West && other == &Direction::East)
        {
            2000 // 180 degree turn costs 2000 points (two 90-degree turns)
        } else {
            1000 // 90 degree turn costs 1000 points
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn all_directions() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
    facing: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    grid: Vec<Vec<char>>,
    start: (i32, i32),
    end: (i32, i32),
}

impl Maze {
    fn from_file(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Failed to read file");
        let grid: Vec<Vec<char>> = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == 'S' {
                    start = (x as i32, y as i32);
                } else if cell == 'E' {
                    end = (x as i32, y as i32);
                }
            }
        }

        Maze { grid, start, end }
    }

    fn is_valid(&self, pos: (i32, i32)) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && pos.1 < self.grid.len() as i32
            && pos.0 < self.grid[0].len() as i32
            && self.grid[pos.1 as usize][pos.0 as usize] != '#'
    }

    fn find_path(&self) -> Option<i32> {
        let mut heap = BinaryHeap::new();
        let mut visited = HashMap::new();

        // Start facing East
        let initial = State {
            cost: 0,
            position: self.start,
            facing: Direction::East,
        };

        heap.push(initial.clone());
        visited.insert((initial.position, initial.facing), 0);

        while let Some(State {
            cost,
            position,
            facing,
        }) = heap.pop()
        {
            if position == self.end {
                return Some(cost);
            }

            // Try all directions
            for new_direction in Direction::all_directions() {
                // Calculate turn cost
                let turn_cost = facing.turn_cost(&new_direction);
                let (dx, dy) = new_direction.delta();
                let new_position = (position.0 + dx, position.1 + dy);

                if self.is_valid(new_position) {
                    let new_cost = cost + turn_cost + 1; // 1000 points for turns, 1 point for movement
                    let key = (new_position, new_direction);

                    if !visited.contains_key(&key) || new_cost < *visited.get(&key).unwrap() {
                        visited.insert(key, new_cost);
                        heap.push(State {
                            cost: new_cost,
                            position: new_position,
                            facing: new_direction,
                        });
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let maze = Maze::from_file("input.txt");
    match maze.find_path() {
        Some(points) => println!("Shortest path found: {} points", points),
        None => println!("No path found!"),
    }
}
