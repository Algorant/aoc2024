use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

// Grid Size
const GRID_SIZE: usize = 50;

#[derive(Debug)]
struct NodePositions {
    character: char,
    positions: Vec<(usize, usize)>,
}

#[derive(Debug)]
struct ValidAntinode {
    node1: (usize, usize),
    node2: (usize, usize),
    antinode: (usize, usize),
    overlapping_char: Option<char>,
}

#[derive(Debug)]
struct NodeOverlap {
    antinode_source_char: char,
    node_char: char,
    position: (usize, usize),
    count: usize,
}

fn is_node_character(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn find_nodes(grid: &Vec<Vec<char>>) -> Vec<NodePositions> {
    let mut node_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    // Find all positions for each character
    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            if is_node_character(ch) {
                node_map.entry(ch).or_insert_with(Vec::new).push((row, col));
            }
        }
    }

    node_map
        .into_iter()
        .map(|(character, positions)| NodePositions {
            character,
            positions,
        })
        .collect()
}

fn is_within_grid(x: i32, y: i32) -> bool {
    x >= 0 && x < GRID_SIZE as i32 && y >= 0 && y < GRID_SIZE as i32
}

fn find_antinodes(
    p1: (usize, usize),
    p2: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> Vec<ValidAntinode> {
    let (x1, y1) = (p1.0 as i32, p1.1 as i32);
    let (x2, y2) = (p2.0 as i32, p2.1 as i32);

    let mut valid_antinodes = Vec::new();

    // Calculate diff vector: pos2 - pos1
    let dx = x2 - x1;
    let dy = y2 - y1;

    // Check if points are the same
    if dx == 0 && dy == 0 {
        return valid_antinodes;
    }

    // Calculate antinode positions using the formula:
    // an1 := pos1 - diff
    // an2 := pos2 + diff
    let antinode1 = (
        x1 - dx, // pos1 - diff
        y1 - dy,
    );

    let antinode2 = (
        x2 + dx, // pos2 + diff
        y2 + dy,
    );

    // Check first antinode
    if is_within_grid(antinode1.0, antinode1.1) {
        let pos = (antinode1.0 as usize, antinode1.1 as usize);
        let overlapping_char = if is_node_character(grid[pos.0][pos.1]) {
            Some(grid[pos.0][pos.1])
        } else {
            None
        };
        valid_antinodes.push(ValidAntinode {
            node1: p1,
            node2: p2,
            antinode: pos,
            overlapping_char,
        });
    }

    // Check second antinode
    if is_within_grid(antinode2.0, antinode2.1) {
        let pos = (antinode2.0 as usize, antinode2.1 as usize);
        let overlapping_char = if is_node_character(grid[pos.0][pos.1]) {
            Some(grid[pos.0][pos.1])
        } else {
            None
        };
        valid_antinodes.push(ValidAntinode {
            node1: p1,
            node2: p2,
            antinode: pos,
            overlapping_char,
        });
    }

    valid_antinodes
}

fn find_all_valid_antinodes(node: &NodePositions, grid: &Vec<Vec<char>>) -> Vec<ValidAntinode> {
    let mut valid_antinodes = Vec::new();

    for i in 0..node.positions.len() {
        for j in i + 1..node.positions.len() {
            valid_antinodes.extend(find_antinodes(node.positions[i], node.positions[j], grid));
        }
    }
    valid_antinodes
}

fn count_node_overlaps(nodes: &[NodePositions], grid: &Vec<Vec<char>>) -> Vec<NodeOverlap> {
    let mut overlap_counts: HashMap<(char, char, (usize, usize)), usize> = HashMap::new();

    for node in nodes {
        let antinodes = find_all_valid_antinodes(node, grid);
        for antinode in antinodes {
            if let Some(overlapping_char) = antinode.overlapping_char {
                if overlapping_char != node.character {
                    // Only count overlaps with different characters
                    let key = (node.character, overlapping_char, antinode.antinode);
                    *overlap_counts.entry(key).or_insert(0) += 1;
                }
            }
        }
    }
    overlap_counts
        .into_iter()
        .map(|((source_char, node_char, pos), count)| NodeOverlap {
            antinode_source_char: source_char,
            node_char,
            position: pos,
            count,
        })
        .collect()
}

// Part 2 function
fn find_line_antinodes(
    p1: (usize, usize),
    p2: (usize, usize),
    grid: &Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    let (x1, y1) = (p1.0 as i32, p1.1 as i32);
    let (x2, y2) = (p2.0 as i32, p2.1 as i32);
    let mut antinodes = Vec::new();

    // Calculate direction vector
    let dx = x2 - x1;
    let dy = y2 - y1;

    if dx == 0 && dy == 0 {
        return antinodes;
    }

    // Include the points themselves as they are now antinodes
    antinodes.push(p1);
    antinodes.push(p2);

    // Extend line in both direction until grid boundaries
    let mut t = 1;
    // Forward direction
    loop {
        let x = x2 + dx * t;
        let y = y2 + dy * t;
        if !is_within_grid(x, y) {
            break;
        }
        antinodes.push((x as usize, y as usize));
        t += 1;
    }

    // Backwards
    let mut t = 1;
    loop {
        let x = x1 - dx * t;
        let y = y1 - dy * t;
        if !is_within_grid(x, y) {
            break;
        }
        antinodes.push((x as usize, y as usize));
        t += 1;
    }
    antinodes
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // Convert input into a 2D grid of characters
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nodes = find_nodes(&grid);

    println!("Found {} unique characters:", nodes.len());
    for node in &nodes {
        println!(
            "Character '{}' appears {} times at positions:",
            node.character,
            node.positions.len()
        );

        if node.positions.len() > 1 {
            let valid_antinodes = find_all_valid_antinodes(node, &grid);
            println!("Valid antinodes within grid for '{}':", node.character);

            for valid in valid_antinodes {
                println!(
                    "  Nodes: ({}, {}) and ({}, {})",
                    valid.node1.0, valid.node1.1, valid.node2.0, valid.node2.1
                );
                println!(
                    "  Valid antinode: ({}, {}){}",
                    valid.antinode.0,
                    valid.antinode.1,
                    if let Some(ch) = valid.overlapping_char {
                        format!(" - overlaps with '{}'", ch)
                    } else {
                        String::new()
                    }
                );
            }
        }
    }

    // Collect all unique antinode locations
    let mut unique_antinode_locations: std::collections::HashSet<(usize, usize)> =
        std::collections::HashSet::new();

    for node in &nodes {
        if node.positions.len() > 1 {
            let antinodes = find_all_valid_antinodes(node, &grid);
            for antinode in antinodes {
                // Add every valid antinode position, regardless of overlaps
                unique_antinode_locations.insert(antinode.antinode);
            }
        }
    }

    println!(
        "\nPart 1 - Total number of unique antinode locations: {}",
        unique_antinode_locations.len()
    );

    // Part 2 print
    let mut all_line_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for node in &nodes {
        if node.positions.len() > 1 {
            for i in 0..node.positions.len() {
                for j in i + 1..node.positions.len() {
                    let line_points =
                        find_line_antinodes(node.positions[i], node.positions[j], &grid);
                    all_line_antinodes.extend(line_points);
                }
            }
        }
    }

    println!(
        "\nPart 2: Total number of antinode locations along lines: {}",
        all_line_antinodes.len()
    );

    // Optional: Print all unique antinode locations (sorted)
    //println!("\nAll unique antinode locations (sorted):");
    //let mut sorted_locations: Vec<_> = unique_antinode_locations.into_iter().collect();
    //sorted_locations.sort_by(|a, b| {
    //    a.0.cmp(&b.0).then(a.1.cmp(&b.1)) // Sort by row first, then by column
    //});
    //for pos in &sorted_locations {
    //    println!("  Position: ({}, {})", pos.0, pos.1);
    //}
}
