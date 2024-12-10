use std::collections::HashSet;
use std::fs::read_to_string;
// Function to identify grid, 47x47 single digit integers

#[derive(Debug)]
struct Grid {
    data: Vec<Vec<u32>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    // Initialize grid
    fn new(input: &str) -> Self {
        let data: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        let rows = data.len();
        let cols = data[0].len();

        Grid { data, rows, cols }
    }

    // Returns the value at the given row and column in the grid if the coordinates are valid,
    // otherwise returns None
    fn get(&self, row: i32, col: i32) -> Option<u32> {
        if row >= 0 && row < self.rows as i32 && col >= 0 && col < self.cols as i32 {
            Some(self.data[row as usize][col as usize])
        } else {
            None
        }
    }

    // Locate trailheads (0 points)
    fn find_trailheads(&self) -> Vec<(i32, i32)> {
        let mut trailheads = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row][col] == 0 {
                    trailheads.push((row as i32, col as i32));
                }
            }
        }
        trailheads
    }

    fn depth_first_search(
        &self,
        pos: (i32, i32),
        current_path: &mut Vec<(i32, i32)>,
        visited: &mut Vec<Vec<bool>>,
        endpoints: &mut HashSet<(i32, i32)>,
        count: &mut usize,
    ) {
        let current_value = self.get(pos.0, pos.1).unwrap();
        let expected_value = current_path.len() as u32;

        // If we're not at the expected number in sequence, stop this path
        if current_value != expected_value {
            return;
        }

        // Add current position to path
        current_path.push(pos);
        visited[pos.0 as usize][pos.1 as usize] = true;

        // If we've reached 9, we've found a valid path
        if current_value == 9 {
            if current_path.len() == 10 && endpoints.insert(pos) {
                // Only count if this is a new endpoint
                *count += 1;
            }
            // Cleanup and return
            current_path.pop();
            visited[pos.0 as usize][pos.1 as usize] = false;
            return;
        }

        // Try all 4 directions (right, down, left, up)
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dr, dc) in directions {
            let next_row = pos.0 + dr;
            let next_col = pos.1 + dc;

            if let Some(next_value) = self.get(next_row, next_col) {
                if !visited[next_row as usize][next_col as usize]
                    && next_value == expected_value + 1
                {
                    self.depth_first_search(
                        (next_row, next_col),
                        current_path,
                        visited,
                        endpoints,
                        count,
                    );
                }
            }
        }

        // Cleanup when backtracking
        current_path.pop();
        visited[pos.0 as usize][pos.1 as usize] = false;
    }

    // Traversal logic to find valid paths (0-9 sequentially using only up down left right)
    // Uses a depth first search implementation above
    fn count_paths(&self, start: (i32, i32)) -> usize {
        let mut count = 0;
        let mut visited = vec![vec![false; self.cols]; self.rows];
        let mut current_path = Vec::new();
        let mut endpoints = HashSet::new();
        self.depth_first_search(
            start,
            &mut current_path,
            &mut visited,
            &mut endpoints,
            &mut count,
        );
        count
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let grid = Grid::new(&input);

    let trailheads = grid.find_trailheads();
    let mut total_paths = 0;

    println!("Analyzing Trailheads:");
    println!("-----------------------");
    for (i, &start) in trailheads.iter().enumerate() {
        let paths = grid.count_paths(start);
        println!(
            "Trailhead #{} at ({}, {}) has {} full paths",
            i + 1,
            start.0,
            start.1,
            paths
        );
        total_paths += paths;
    }
    println!("-----------------------");

    println!("Total number of full paths: {}", total_paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_grid() {
        let input = "89010123\n\
                    78121874\n\
                    87430965\n\
                    96549874\n\
                    45678903\n\
                    32019012\n\
                    01329801\n\
                    10456732";

        let grid = Grid::new(input);
        let trailheads = grid.find_trailheads();

        // Test each trailhead's paths
        let mut scores = Vec::new();
        for &start in &trailheads {
            let paths = grid.count_paths(start);
            scores.push(paths);
        }

        // The example should have 9 trailheads with these scores
        assert_eq!(scores, vec![5, 6, 5, 3, 1, 3, 5, 3, 5]);

        // Total should be 36
        assert_eq!(scores.iter().sum::<usize>(), 36);
    }
}
