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

fn find_perimeter(region: &Region, grid: &Vec<Vec<char>>) -> usize {
    let mut perimiter = 0;

    for &(row, col) in &region.positions {
        // Check all 4 sides of each position
        let directions = [
            (row.wrapping_sub(1), col),
            (row, col + 1),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
        ];

        for (next_row, next_col) in directions {
            // A side counts if it is on the edge of the grid
            // or the adjacent cell is not part of the region
            if next_row >= grid.len()
                || next_col >= grid[0].len()
                || !region.positions.contains(&(next_row, next_col))
            {
                perimiter += 1;
            }
        }
    }
    perimiter
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
    let total_sum: usize = regions
        .iter()
        .map(|region| {
            let perimeter = find_perimeter(region, &grid);
            let area = region.positions.len();

            println!(
                "Region {} - Letter: {}, Area: {} tiles, Perimeter: {} Sides",
                regions
                    .iter()
                    .position(|r| r.letter == region.letter)
                    .unwrap()
                    + 1,
                region.letter,
                area,
                perimeter
            );
            area * perimeter
        })
        .sum();

    println!("\nSum of all regions' (area * perimeter): {}", total_sum);
}
