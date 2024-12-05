use std::fs::read_to_string;

fn find_xmas(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Helper for sequence
    fn is_xmas(chars: &[char]) -> bool {
        chars == &['X', 'M', 'A', 'S'] || chars == &['S', 'A', 'M', 'X']
    }

    // Check horizontal
    for row in 0..height {
        for col in 0..=width - 4 {
            let chars: Vec<char> = (0..4).map(|i| grid[row][col + i]).collect();
            if is_xmas(&chars) {
                count += 1;
            }
        }
    }

    // Check vertical (top to bottom and bottom to top)
    for row in 0..=height - 4 {
        for col in 0..width {
            let chars: Vec<char> = (0..4).map(|i| grid[row + i][col]).collect();
            if is_xmas(&chars) {
                count += 1
            }
        }
    }

    // Check diagonal
    for row in 0..=height - 4 {
        for col in 0..=width - 4 {
            let chars: Vec<char> = (0..4).map(|i| grid[row + i][col + i]).collect();
            if is_xmas(&chars) {
                count += 1;
            }
        }
    }

    // check diagonal (bottom-left to top-right)
    for row in 3..height {
        for col in 0..=width - 4 {
            let chars: Vec<char> = (0..4).map(|i| grid[row - i][col + i]).collect();
            if is_xmas(&chars) {
                count += 1;
            }
        }
    }
    count
}

// Part 2, finding the X-Mas shape
fn find_xmas_x(grid: &Vec<Vec<char>>) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Need at least 3x3 space for the X pattern
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            // Check center A
            if grid[row][col] != 'A' {
                continue;
            }

            // Check the four endpoints make an X with M and S
            let top_left = grid[row - 1][col - 1];
            let top_right = grid[row - 1][col + 1];
            let bottom_left = grid[row + 1][col - 1];
            let bottom_right = grid[row + 1][col + 1];

            // Check all valid X patterns:
            // Pattern 1: M M / S S
            // Pattern 2: S S / M M
            // Pattern 3: M S / M S
            // Pattern 4: S M / S M
            if (top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'M')
                || (top_left == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M'
                    && bottom_right == 'S')
                || (top_left == 'S'
                    && top_right == 'M'
                    && bottom_left == 'S'
                    && bottom_right == 'M')
            {
                count += 1;
                // To Debug inputs:
                //println!("Found X pattern at row {}, col {}", row, col);
            }
        }
    }
    count
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // convert to grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Find Xmas Occurences
    let count = find_xmas(&grid);
    println!("Found {} occurences of XMAS pattern", count);

    // Find X-MAS shapes
    let x_count = find_xmas_x(&grid);
    println!("Found {} X-MAS shapes", x_count);
}
