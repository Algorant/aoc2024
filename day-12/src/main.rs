use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Region {
    letter: char,
    positions: HashSet<(usize, usize)>,
}

fn flood_fill(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    letter: char,
    region: &mut HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
) {
    if visited.contains(&(row, col)) || grid[row][col] != letter {
        return;
    }
    visited.insert((row, col));
    region.insert((row, col));

    // Check all adjacent tiles
    let directions = [
        (row.wrapping_sub(1), col),
        (row, col + 1),
        (row + 1, col),
        (row, col.wrapping_sub(1)),
    ];

    for (next_row, next_col) in directions {
        if next_row < grid.len() && next_col < grid[0].len() {
            flood_fill(grid, next_row, next_col, letter, region, visited);
        }
    }
}

fn find_regions(grid: &Vec<Vec<char>>) -> Vec<Region> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if !visited.contains(&(row, col)) {
                let letter = grid[row][col];
                let mut region_positions = HashSet::new();

                // Flood fill to find all connected positions with same letter
                flood_fill(grid, row, col, letter, &mut region_positions, &mut visited);

                if !region_positions.is_empty() {
                    regions.push(Region {
                        letter,
                        positions: region_positions,
                    })
                }
            }
        }
    }

    regions
}

// Area is defined as the number of tiles in a region.

// Perimiter is defined as the number of sides of a tile
// not connected to the same type of tile in a region.

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // Convert to grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let regions = find_regions(&grid);

    // Print information about each region
    for (i, region) in regions.iter().enumerate() {
        println!(
            "Region {} - Letter: {}, Size: {} tiles",
            i + 1,
            region.letter,
            region.positions.len()
        );
    }
}
