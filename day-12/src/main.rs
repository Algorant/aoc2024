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

// Part 2
fn count_distinct_sides(region: &Region, grid: &Vec<Vec<char>>) -> usize {
    let mut distinct_sides = 0;
    let mut horizontal_edges = HashSet::new();
    let mut vertical_edges = HashSet::new();

    // For each position in the region
    for &(row, col) in &region.positions {
        // Check horizontal edges (top and bottom)
        // Top edge
        if !region.positions.contains(&(row.wrapping_sub(1), col)) {
            if !horizontal_edges.contains(&(row, col)) {
                let mut current_col = col;
                // Find the full length of this horizontal edge
                while current_col < grid[0].len()
                    && region.positions.contains(&(row, current_col))
                    && !region
                        .positions
                        .contains(&(row.wrapping_sub(1), current_col))
                {
                    horizontal_edges.insert((row, current_col));
                    current_col += 1;
                }
                distinct_sides += 1;
            }
        }

        // Bottom edge
        if row + 1 >= grid.len() || !region.positions.contains(&(row + 1, col)) {
            if !horizontal_edges.contains(&(row + 1, col)) {
                let mut current_col = col;
                while current_col < grid[0].len()
                    && region.positions.contains(&(row, current_col))
                    && (row + 1 >= grid.len()
                        || !region.positions.contains(&(row + 1, current_col)))
                {
                    horizontal_edges.insert((row + 1, current_col));
                    current_col += 1;
                }
                distinct_sides += 1;
            }
        }

        // Check vertical edges (left and right)
        // Left edge
        if !region.positions.contains(&(row, col.wrapping_sub(1))) {
            if !vertical_edges.contains(&(row, col)) {
                let mut current_row = row;
                while current_row < grid.len()
                    && region.positions.contains(&(current_row, col))
                    && !region
                        .positions
                        .contains(&(current_row, col.wrapping_sub(1)))
                {
                    vertical_edges.insert((current_row, col));
                    current_row += 1;
                }
                distinct_sides += 1;
            }
        }

        // Right edge
        if col + 1 >= grid[0].len() || !region.positions.contains(&(row, col + 1)) {
            if !vertical_edges.contains(&(row, col + 1)) {
                let mut current_row = row;
                while current_row < grid.len()
                    && region.positions.contains(&(current_row, col))
                    && (col + 1 >= grid[0].len()
                        || !region.positions.contains(&(current_row, col + 1)))
                {
                    vertical_edges.insert((current_row, col + 1));
                    current_row += 1;
                }
                distinct_sides += 1;
            }
        }
    }

    distinct_sides
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    // Convert to grid
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let regions = find_regions(&grid);

    // Print information about each region and calculate both parts
    let (part1_sum, part2_sum): (usize, usize) = regions
        .iter()
        .map(|region| {
            let perimeter = find_perimeter(region, &grid);
            let area = region.positions.len();
            let distinct_sides = count_distinct_sides(region, &grid);

            println!(
                "Region {} - Letter: {}, Area: {} tiles, Perimeter: {} sides, Distinct sides: {}",
                regions
                    .iter()
                    .position(|r| r.letter == region.letter)
                    .unwrap()
                    + 1,
                region.letter,
                area,
                perimeter,
                distinct_sides
            );
            (area * perimeter, area * distinct_sides)
        })
        .fold((0, 0), |acc, (p1, p2)| (acc.0 + p1, acc.1 + p2));

    println!(
        "\nPart 1 - Sum of all regions' (area * perimeter): {}",
        part1_sum
    );
    println!(
        "Part 2 - Sum of all regions' (area * distinct sides): {}",
        part2_sum
    );
}
