use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Rock,
    Robot,
}

#[derive(Debug, Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    robot_pos: (usize, usize),
    rocks: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut robot_pos = (0, 0);
        let mut rocks = HashSet::new();

        if input.trim().is_empty() {
            panic!("Input is empty!");
        }

        println!("Parsing map with {} lines", input.lines().count());
        println!("First line of input: {:?}", input.lines().next().unwrap());

        for (y, line) in input.lines().enumerate() {
            //println!("Line {}: length={}", y, line.len());
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let cell = match ch {
                    '.' => Cell::Empty,
                    '#' => Cell::Wall,
                    'O' => {
                        rocks.insert((x, y));
                        Cell::Rock
                    }
                    '@' => {
                        println!("Found robot at position ({}, {})", x, y);
                        robot_pos = (x, y);
                        Cell::Robot
                    }
                    _ => {
                        println!("Found unexpected character '{}' at ({}, {})", ch, x, y);
                        Cell::Empty
                    }
                };
                row.push(cell);
            }
            cells.push(row);
        }

        let height = cells.len();
        let width = cells.get(0).map_or(0, |row| row.len());

        println!("Grid dimensions: {}x{}", width, height);
        println!("Number of rocks: {}", rocks.len());
        println!("Robot position: {:?}", robot_pos);

        Grid {
            cells,
            robot_pos,
            rocks,
            width,
            height,
        }
    }

    fn move_robot(&mut self, direction: char) -> bool {
        let (dx, dy) = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid direction: {}", direction),
        };

        let (x, y) = self.robot_pos;
        let new_x = (x as i32 + dx) as usize;
        let new_y = (y as i32 + dy) as usize;

        // Check Bounds
        if new_x >= self.width || new_y >= self.height {
            return false;
        }

        match self.cells[new_y][new_x] {
            Cell::Empty => {
                self.cells[y][x] = Cell::Empty;
                self.cells[new_y][new_x] = Cell::Robot;
                self.robot_pos = (new_x, new_y);
                true
            }
            Cell::Rock => {
                // Find all consecutive rocks and the first non-rock cell
                let mut rocks_to_move = Vec::new();
                let mut curr_x = new_x;
                let mut curr_y = new_y;

                loop {
                    if self.cells[curr_y][curr_x] == Cell::Rock {
                        rocks_to_move.push((curr_x, curr_y));
                        let next_x = (curr_x as i32 + dx) as usize;
                        let next_y = (curr_y as i32 + dy) as usize;

                        if next_x >= self.width || next_y >= self.height {
                            return false;
                        }
                        curr_x = next_x;
                        curr_y = next_y;
                    } else if self.cells[curr_y][curr_x] == Cell::Wall {
                        return false; // Stop if we hit a wall
                    } else {
                        break; // Break if we hit empty space
                    }
                }

                // Check if we can move all rocks (space after last rock is empty)
                println!(
                    "Attempting to move {} rocks in direction {}",
                    rocks_to_move.len(),
                    direction
                );
                if self.cells[curr_y][curr_x] == Cell::Empty {
                    // Move all rocks one position in the direction
                    for &(rock_x, rock_y) in rocks_to_move.iter().rev() {
                        let new_rock_x = (rock_x as i32 + dx) as usize;
                        let new_rock_y = (rock_y as i32 + dy) as usize;

                        // Verify the rock is still where we expect it
                        if self.cells[rock_y][rock_x] != Cell::Rock {
                            continue; // Skip if rock is no longer here
                        }

                        self.cells[rock_y][rock_x] = Cell::Empty;
                        self.cells[new_rock_y][new_rock_x] = Cell::Rock;

                        // Update the rocks set
                        if self.rocks.remove(&(rock_x, rock_y)) {
                            self.rocks.insert((new_rock_x, new_rock_y));
                        }
                    }

                    // Move robot
                    self.cells[y][x] = Cell::Empty;
                    self.cells[new_y][new_x] = Cell::Robot;

                    self.robot_pos = (new_x, new_y);

                    true
                } else {
                    false
                }
            }
            Cell::Wall => false,
            Cell::Robot => false, // Can't move into a cell with another robot
        }
    }

    fn verify_grid_state(&self) -> bool {
        let mut rock_count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == Cell::Rock {
                    rock_count += 1;
                    if !self.rocks.contains(&(x, y)) {
                        println!("Rock at ({}, {}) not in rocks set!", x, y);
                        return false;
                    }
                }
            }
        }
        if rock_count != self.rocks.len() {
            println!(
                "Rock count mismatch! Grid: {}, Set: {}",
                rock_count,
                self.rocks.len()
            );
            return false;
        }
        true
    }

    fn execute_moves(&mut self, moves: &str) {
        for direction in moves.chars().filter(|c| !c.is_whitespace()) {
            match direction {
                '<' | '>' | '^' | 'v' => {
                    let moved = self.move_robot(direction);
                    if !moved {
                        println!("Could not move in direction: {}", direction);
                    }
                    if !self.verify_grid_state() {
                        panic!("Grid state verification failed after moving {}", direction);
                    }
                }
                _ => println!("Skipping invalid direction: {}", direction),
            }
        }
    }

    fn get_rock_positions(&self) -> &HashSet<(usize, usize)> {
        &self.rocks
    }

    fn calculate_score(&self) -> usize {
        self.rocks.iter().fold(0, |acc, &(x, y)| {
            let y_score = 100 * y; // Remove the +1 from y calculation
            let x_score = x; // Remove the +1 from x calculation
            acc + y_score + x_score
        })
    }

    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = &self.cells[y][x];
                match cell {
                    Cell::Empty => print!("."),
                    Cell::Wall => print!("#"),
                    Cell::Rock => print!("O"),
                    Cell::Robot => print!("@"),
                }
            }
            println!();
        }
        println!();
    }
}

fn main() {
    let map_str = read_to_string("map.txt").expect("Failed to read map file");
    let moves = read_to_string("movements.txt").expect("Failed to read movement file");

    println!(
        "First few moves: {:?}",
        moves.trim().chars().take(10).collect::<String>()
    );

    let mut grid = Grid::new(&map_str);

    // Print initial state
    println!("Initial robot position: {:?}", grid.robot_pos);
    println!("Initial number of rocks: {}", grid.rocks.len());
    println!("\nInitial grid state:");
    grid.print_grid();

    grid.execute_moves(&moves.trim());

    println!("\nFinal grid state:");
    grid.print_grid();

    println!("Final robot position: {:?}", grid.robot_pos);
    println!("Final number of rocks: {}", grid.rocks.len());
    println!("Final rock positions: {:?}", grid.get_rock_positions());
    println!("Total score: {}", grid.calculate_score());
}
