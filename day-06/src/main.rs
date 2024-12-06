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

fn count_moves(grid: &Vec<Vec<char>>) -> usize {
    let mut moves = 0;
    let height = grid.len();

    // Find starting position
    let (mut current_row, current_col) = match find_start_position(&grid) {
        Some(pos) => pos,
        None => {
            println!("No starting position (^) found!");
            return 0;
        }
    };

    while current_row > 0 {
        // Check next position up
        let next_row = current_row - 1;

        // If hit obstacle, stop
        if grid[next_row][current_col] == '#' {
            break;
        }

        // Move to next position
        current_row = next_row;
        moves += 1;

        println!("Moved to: ({}, {})", current_row, current_col);
    }

    moves
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // Convert input to a 2d grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let total_moves = count_moves(&grid);
    println!("Total moves made: {}", total_moves);
}
