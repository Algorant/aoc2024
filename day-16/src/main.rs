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
struct PathState {
    cost: i32,
    position: (i32, i32),
    facing: Direction,
    path: Vec<(i32, i32)>,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for PathState {
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

    fn find_paths(&self) -> Option<(i32, usize, Vec<Vec<(i32, i32)>>)> {
        let mut heap = BinaryHeap::new();
        let mut visited = HashMap::new();
        let mut min_cost = i32::MAX;
        let mut unique_positions = std::collections::HashSet::new();
        let mut all_min_cost_paths = Vec::new();

        // Start facing East
        let initial = PathState {
            cost: 0,
            position: self.start,
            facing: Direction::East,
            path: vec![self.start],
        };

        heap.push(initial.clone());
        visited.insert((initial.position, initial.facing), 0);

        while let Some(PathState {
            cost,
            position,
            facing,
            path,
        }) = heap.pop()
        {
            // Skip paths that are already worse than our best
            if min_cost != i32::MAX && cost > min_cost {
                continue;
            }

            if position == self.end {
                if cost < min_cost {
                    // Found a better path, reset everything
                    min_cost = cost;
                    all_min_cost_paths.clear();
                    all_min_cost_paths.push(path.clone());
                } else if cost == min_cost {
                    // Found another path with same cost
                    all_min_cost_paths.push(path.clone());
                }
                continue;
            }

            // Try all directions
            for new_direction in Direction::all_directions() {
                let turn_cost = facing.turn_cost(&new_direction);
                let (dx, dy) = new_direction.delta();
                let new_position = (position.0 + dx, position.1 + dy);

                if self.is_valid(new_position) {
                    let new_cost = cost + turn_cost + 1;
                    let key = (new_position, new_direction);

                    // Allow paths with equal cost to continue
                    if !visited.contains_key(&key) || new_cost <= *visited.get(&key).unwrap() {
                        // Only update the visited cost if it's better
                        if !visited.contains_key(&key) || new_cost < *visited.get(&key).unwrap() {
                            visited.insert(key, new_cost);
                        }
                        let mut new_path = path.clone();
                        new_path.push(new_position);
                        heap.push(PathState {
                            cost: new_cost,
                            position: new_position,
                            facing: new_direction,
                            path: new_path,
                        });
                    }
                }
            }
        }

        if min_cost != i32::MAX {
            // After finding all minimum cost paths, collect all unique positions
            unique_positions.clear();

            // Explicitly ensure S and E are included
            unique_positions.insert(self.start);
            unique_positions.insert(self.end);

            // Add all positions from all optimal paths
            for path in &all_min_cost_paths {
                unique_positions.extend(path.iter().cloned());
            }

            // Double check S and E are included
            if !unique_positions.contains(&self.start) || !unique_positions.contains(&self.end) {
                println!("WARNING: Start or End position missing from unique positions!");
            }
            Some((min_cost, unique_positions.len(), all_min_cost_paths))
        } else {
            None
        }
    }
}

fn main() {
    let maze = Maze::from_file("input.txt");
    match maze.find_paths() {
        Some((points, unique_positions, paths)) => {
            println!("Shortest path found: {} points", points);

            println!(
                "Number of unique positions in shortest path(s): {} (including S and E)",
                unique_positions
            );

            // Print the maze with marked paths
            let mut marked = vec![vec!['.'; maze.grid[0].len()]; maze.grid.len()];

            // Mark walls
            for y in 0..maze.grid.len() {
                for x in 0..maze.grid[0].len() {
                    if maze.grid[y][x] == '#' {
                        marked[y][x] = '#';
                    }
                }
            }

            // Mark all positions that are part of any optimal path
            let mut path_positions = std::collections::HashSet::new();
            for path in paths {
                for &(x, y) in &path {
                    path_positions.insert((x, y));
                }
            }

            // Verify S and E are included in paths
            let start_included = path_positions.contains(&maze.start);
            let end_included = path_positions.contains(&maze.end);
            println!("Start position included: {}", start_included);
            println!("End position included: {}", end_included);

            // Mark path positions with 'O'
            for &(x, y) in &path_positions {
                if marked[y as usize][x as usize] != '#' {
                    marked[y as usize][x as usize] = 'O';
                }
            }

            // Mark start and end
            marked[maze.start.1 as usize][maze.start.0 as usize] = 'S';
            marked[maze.end.1 as usize][maze.end.0 as usize] = 'E';

            // Print the marked maze
            println!("\nMaze with optimal paths marked ('O'):");
            for row in marked {
                println!("{}", row.iter().collect::<String>());
            }
        }
        None => println!("No path found!"),
    }
}
