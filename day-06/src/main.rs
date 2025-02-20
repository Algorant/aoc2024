use std::collections::HashSet;
use std::fs::read_to_string;

fn find_start_position(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if ch == '^' {
                return Some((row, col));
            }
        }
    }
    None
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_position(&self, row: usize, col: usize) -> (isize, isize) {
        match self {
            Direction::Up => (row as isize - 1, col as isize),
            Direction::Right => (row as isize, col as isize + 1),
            Direction::Down => (row as isize + 1, col as isize),
            Direction::Left => (row as isize, col as isize - 1),
        }
    }
}

// Create Another struct to track State of the character
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    row: usize,
    col: usize,
    direction: Direction,
}

fn simulate_path(grid: &Vec<Vec<char>>) -> Option<bool> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited_states = HashSet::new();

    // Find starting position
    let (mut current_row, mut current_col) = match find_start_position(&grid) {
        Some(pos) => pos,
        None => return None,
    };

    let mut direction = Direction::Up;

    loop {
        // Create current state
        let current_state = State {
            row: current_row,
            col: current_col,
            direction,
        };

        // Check if this is a state we have been in before (loop detection!)
        if !visited_states.insert(current_state) {
            return Some(true); // Loop was found
        }

        // Get next position
        let (next_row, next_col) = direction.get_next_position(current_row, current_col);

        // Check if off the grid
        if next_row < 0 || next_row >= height as isize || next_col < 0 || next_col >= width as isize
        {
            return Some(false); // Path is off the grid
        }

        // Convert to usize
        let next_row = next_row as usize;
        let next_col = next_col as usize;

        // Handle obstacles
        if grid[next_row][next_col] == '#' {
            direction = direction.turn_right();
            continue;
        }

        current_row = next_row;
        current_col = next_col;
    }
}

fn find_loop_creating_positions(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut loop_positions = Vec::new();

    // Try each empty position
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == '.' {
                // Create new grid with obstacle at this position
                let mut test_grid = grid.clone();
                test_grid[row][col] = '#';

                // Check if loop created
                if let Some(true) = simulate_path(&test_grid) {
                    loop_positions.push((row, col));
                }
            }
        }
    }

    loop_positions
}

fn count_moves(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = HashSet::new();

    // Find starting position
    let (mut current_row, mut current_col) = match find_start_position(&grid) {
        Some(pos) => pos,
        None => {
            println!("No starting position (^) found!");
            return 0;
        }
    };

    // Add first position to visited set
    visited.insert((current_row, current_col));

    // Initial Direction
    let mut direction = Direction::Up;

    loop {
        // Get next position based on current direction
        let (next_row, next_col) = direction.get_next_position(current_row, current_col);

        // First check if next position would be off the grid
        if next_row < 0 || next_row >= height as isize || next_col < 0 || next_col >= width as isize
        {
            break;
        }

        // Convert to usize now that it is known to be inbounds
        let next_row = next_row as usize;
        let next_col = next_col as usize;

        // Look ahead for obstacle
        if grid[next_row][next_col] == '#' {
            // Don't move, just turn right and continue
            direction = direction.turn_right();
            continue;
        }

        // No obstacle ahead, safe to move
        current_row = next_row;
        current_col = next_col;
        visited.insert((current_row, current_col));

        println!(
            "Moved to: ({}, {}, Direction: {:?})",
            current_row, current_col, direction
        );
    }

    visited.len()
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // Convert input to a 2d grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Part 1: Count Distinct Positions
    let total_moves = count_moves(&grid);
    println!("Part 1 - Total distinct positions: {}", total_moves);

    // Part 2: Find positions that create loops
    let loop_positions = find_loop_creating_positions(&grid);
    println!(
        "Part 2 - Number of loop creation points: {}",
        loop_positions.len()
    );
}
