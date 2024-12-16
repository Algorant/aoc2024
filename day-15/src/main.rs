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

        for (y, line) in input.lines().enumerate() {
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
                        robot_pos = (x, y);
                        Cell::Robot
                    }
                    _ => panic!("Invalid character in grid: {}", ch),
                };
                row.push(cell);
            }
            cells.push(row);
        }

        let height = cells.len();
        let width = cells.get(0).map_or(0, |row| row.len());

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
                // Try to push rock
                let rock_new_x = (new_x as i32 + dx) as usize;
                let rock_new_y = (new_y as i32 + dy) as usize;

                if rock_new_x >= self.width || rock_new_y >= self.height {
                    return false;
                }

                if self.cells[rock_new_y][rock_new_x] == Cell::Empty {
                    // Move rock
                    self.cells[new_y][new_x] = Cell::Robot;
                    self.cells[rock_new_y][rock_new_x] = Cell::Empty;
                    self.robot_pos = (new_x, new_y);
                    self.rocks.remove(&(new_x, new_y));
                    self.rocks.insert((rock_new_x, rock_new_y));
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn execute_moves(&mut self, moves: &str) {
        for direction in moves.chars() {
            self.move_robot(direction);
        }
    }

    fn get_rock_positions(&self) -> &HashSet<(usize, usize)> {
        &self.rocks
    }
}

fn parse_input(input: &str) -> (String, String) {
    // Find the map part (everything up to the line of all #'s)
    let lines: Vec<&str> = input.lines().collect();

    // Find the border line (all #'s)
    let border_index = lines
        .iter()
        .position(|line| line.chars().all(|c| c == '#'))
        .expect("Could not find border line");

    // Map is everything up to and including the border
    let map = lines[..=border_index].join("\n");

    // Moves are everything after the border, joined and trimmed
    let moves = lines[border_index + 1..]
        .iter()
        .map(|&line| line)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("");

    (map, moves.trim().to_string())
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");
    let (map_str, moves) = parse_input(&input);

    let mut grid = Grid::new(&map_str);
    grid.execute_moves(&moves);

    println!("Final rock positions: {:?}", grid.get_rock_positions());
}
