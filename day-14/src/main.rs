use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
    initial_position: (i32, i32),
}

impl Robot {
    fn from_line(line: &str) -> Self {
        // Parse lines
        let parts: Vec<&str> = line.split(' ').collect();

        // Parse position
        let pos_str = parts[0].trim_start_matches("p=");
        let pos_parts: Vec<i32> = pos_str.split(',').map(|n| n.parse().unwrap()).collect();

        // Parse velocity
        let vel_str = parts[1].trim_start_matches("v=");
        let vel_parts: Vec<i32> = vel_str.split(',').map(|n| n.parse().unwrap()).collect();

        let position = (pos_parts[0], pos_parts[1]);
        Robot {
            position,
            initial_position: position,
            velocity: (vel_parts[0], vel_parts[1]),
        }
    }

    fn step(&mut self, width: i32, height: i32) {
        // Update position based on velocity
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Apply wrapping
        self.position.0 = self.wrap(self.position.0, width);
        self.position.1 = self.wrap(self.position.1, height);
    }

    fn wrap(&self, value: i32, limit: i32) -> i32 {
        if value < 0 {
            value + limit
        } else if value >= limit {
            value % limit
        } else {
            value
        }
    }

    fn simulate_steps(&mut self, steps: usize, width: i32, height: i32) -> Vec<(i32, i32)> {
        let mut positions = Vec::with_capacity(steps);
        positions.push(self.position);

        for _ in 0..steps {
            self.step(width, height);
            positions.push(self.position);
        }

        positions
    }

    fn reset(&mut self) {
        self.position = self.initial_position;
    }
}

fn simulate_robots(
    robots: &mut Vec<Robot>,
    steps: usize,
    width: i32,
    height: i32,
) -> Vec<Vec<(i32, i32)>> {
    robots
        .iter_mut()
        .map(|robot| {
            let positions = robot.simulate_steps(steps, width, height);
            robot.reset(); // Reset robot to initial position after simulation
            positions
        })
        .collect()
}

// Helper functions for quadrants and calculating robots therein
fn get_quadrant(pos: (i32, i32), width: i32, height: i32) -> Option<usize> {
    let mid_x = width / 2;
    let mid_y = height / 2;

    // If robot is on midpoint for any quadrant, return None
    if pos.0 == mid_x || pos.1 == mid_y {
        return None;
    }

    match (pos.0 < mid_x, pos.1 < mid_y) {
        (true, true) => Some(0),   // top-left quadrant
        (false, true) => Some(1),  // Top-right quadrant
        (true, false) => Some(2),  // Bottom-left quadrant
        (false, false) => Some(3), // Bottom-right quadrant
    }
}

fn count_robots_in_quadrants(
    positions: &[Vec<(i32, i32)>],
    width: i32,
    height: i32,
    step: usize,
) -> [usize; 4] {
    let mut quadrant_counts = [0; 4];
    let mut middle_count = 0;

    for robot_positions in positions {
        if let Some(pos) = robot_positions.get(step) {
            if let Some(quadrant) = get_quadrant(*pos, width, height) {
                quadrant_counts[quadrant] += 1;
            } else {
                middle_count += 1;
            }
        }
    }
    println!("Robots on dividing lines: {}", middle_count);
    quadrant_counts
}

// Part 2 functions, detect christmas tree pattern
// Use a hashmap to find step where all robots are in unique positions
fn find_unique_positions_steps(positions: &[Vec<(i32, i32)>], max_steps: usize) -> Vec<usize> {
    let mut unique_steps = Vec::new();

    for step in 0..max_steps {
        let mut positions_set = HashSet::new();
        let mut all_unique = true;

        // Check positions at this step
        for robot_positions in positions {
            if let Some(pos) = robot_positions.get(step) {
                // If we can't insert the position, it means it's already in the set
                if !positions_set.insert(*pos) {
                    all_unique = false;
                    break;
                }
            }
        }

        if all_unique {
            unique_steps.push(step);
            if unique_steps.len() >= 5 {
                break;
            }
        }
    }
    unique_steps
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let mut robots: Vec<Robot> = input.lines().map(Robot::from_line).collect();

    // Grid dimensions
    let width = 101;
    let height = 103;

    // Count robots in each quadrant after 100 steps aka Part 1
    let steps = 100;
    let all_positions = simulate_robots(&mut robots, steps, width, height);
    let quadrant_counts = count_robots_in_quadrants(&all_positions, width, height, steps);

    // Print results
    println!("Part 1");
    println!("===================================");
    println!("\nAfter {} steps:", steps);
    println!("Quadrant counts:");
    println!("Top-left (Q1): {} robots", quadrant_counts[0]);
    println!("Top-right (Q2): {} robots", quadrant_counts[1]);
    println!("Bottom-left (Q3): {} robots", quadrant_counts[2]);
    println!("Bottom-right (Q4): {} robots", quadrant_counts[3]);
    println!(
        "Safety factor: {}",
        quadrant_counts[0] * quadrant_counts[1] * quadrant_counts[2] * quadrant_counts[3]
    );
    println!("\nPart 2");
    println!("===================================");

    // Simulate for longer to find box pattern
    let max_steps = width as usize * height as usize; // Since width and height are coprime, their LCM is their product
    let all_positions = simulate_robots(&mut robots, max_steps, width, height);

    // Find the first 5 steps where all robots have unique positions
    let unique_steps = find_unique_positions_steps(&all_positions, max_steps);

    if !unique_steps.is_empty() {
        println!(
            "\nFirst {} steps where all robots have unique positions:",
            unique_steps.len()
        );
        for (i, step) in unique_steps.iter().enumerate() {
            println!("\nStep #{}: {}", i + 1, step);

            // Visualize the positions at this step
            let mut grid = vec![vec!['.'; width as usize]; height as usize];
            for robot_positions in &all_positions {
                if let Some(pos) = robot_positions.get(*step) {
                    let x = pos.0 as usize;
                    let y = pos.1 as usize;
                    if x < width as usize && y < height as usize {
                        grid[y][x] = '#';
                    }
                }
            }

            println!("\nPattern visualization:");
            for row in grid {
                println!("{}", row.iter().collect::<String>());
            }
        }
    } else {
        println!(
            "\nNo steps found where all robots have unique positions within {} steps",
            max_steps
        );
    }
}
