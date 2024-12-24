use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
    path: Vec<(usize, usize)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut path = Vec::new();
        let mut current = (0, 0);
        let mut found_path = false;

        // Parse the grid
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    'S' => {
                        start = (x, y);
                        current = (x, y);
                        path.push((x, y));
                    }
                    'E' => end = (x, y),
                    _ => (),
                }
                row.push(ch);
            }
            cells.push(row);
        }

        // Find the single path from S to E
        while !found_path {
            let (x, y) = current;
            // Try each direction
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;

                if new_x < cells[0].len()
                    && new_y < cells.len()
                    && cells[new_y][new_x] != '#'
                    && !path.contains(&(new_x, new_y))
                {
                    path.push((new_x, new_y));
                    current = (new_x, new_y);
                    if current == end {
                        found_path = true;
                    }
                    break;
                }
            }
        }

        let height = cells.len();
        let width = cells.get(0).map_or(0, |row| row.len());

        Grid {
            cells,
            width,
            height,
            start,
            end,
            path,
        }
    }

    fn find_wall_shortcuts(&self) -> Vec<((usize, usize), usize)> {
        let mut shortcuts = Vec::new();
        let normal_length = self.path.len() - 1; // -1 because we count steps, not positions

        // For each position in the path
        for (path_idx, &(x, y)) in self.path.iter().enumerate() {
            // Check adjacent walls
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let wall_x = (x as i32 + dx) as usize;
                let wall_y = (y as i32 + dy) as usize;

                // Skip if out of bounds
                if wall_x >= self.width || wall_y >= self.height {
                    continue;
                }

                // If it's a wall
                if self.cells[wall_y][wall_x] == '#' {
                    // Look for a path position that this wall connects to
                    for (dest_idx, &dest_pos) in self.path.iter().enumerate() {
                        // Skip if we're looking at positions before current path position
                        if dest_idx <= path_idx {
                            continue;
                        }

                        // Check if the wall is adjacent to this path position
                        if ((dest_pos.0 as i32 - wall_x as i32).abs()
                            + (dest_pos.1 as i32 - wall_y as i32).abs())
                            == 1
                        {
                            // Calculate how many steps this shortcut would save
                            let shortcut_length = path_idx + 2; // +2 for the wall and next position
                            let original_length = dest_idx - path_idx;
                            let saved_steps = original_length - 1; // -1 because we're adding one step through wall

                            if saved_steps >= 100 {
                                shortcuts.push(((wall_x, wall_y), saved_steps));
                            }
                        }
                    }
                }
            }
        }

        shortcuts
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let grid = Grid::new(&input);

    // Find all wall shortcuts that save at least 100 moves
    let shortcuts = grid.find_wall_shortcuts();

    // Group shortcuts by number of steps saved and count them
    let mut counts: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
    for (_, saved_steps) in shortcuts {
        *counts.entry(saved_steps).or_insert(0) += 1;
    }

    // Sum up all the counts (ignoring the saved_steps values)
    let total_points: usize = counts.values().sum();
    println!(
        "Total number of points that save >100 steps: {}",
        total_points
    );
}
