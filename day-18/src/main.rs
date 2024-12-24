use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq)]
struct Node {
    point: Point,
    steps: i32,
    f_score: i32,
}

// Custom ordering for priority queue
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_score
            .cmp(&self.f_score)
            .then_with(|| other.steps.cmp(&self.steps))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn read_danger_points() -> io::Result<Vec<Point>> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut points = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((x, y)) = line.split_once(',') {
            if let (Ok(x), Ok(y)) = (x.parse(), y.parse()) {
                points.push(Point { x, y });
            }
        }
    }
    Ok(points)
}

fn is_valid_point(p: &Point, size: i32) -> bool {
    p.x >= 0 && p.x <= size && p.y >= 0 && p.y <= size // Changed to include 70
}

fn find_shortest_path(size: i32, danger_points: Vec<Point>) -> Option<i32> {
    let start = Point { x: 0, y: 0 };
    let goal = Point { x: size, y: size };
    let mut open_set = BinaryHeap::new();
    let mut closed_set = HashSet::new();

    // Only use the first 1024 danger points
    let dangers: HashSet<Point> = danger_points.iter().take(1024).copied().collect();

    // Initialize with starting point
    open_set.push(Node {
        point: start,
        steps: 0,
        f_score: manhattan_distance(&start, &goal),
    });

    // Possible moves: up, down, left, right
    let directions = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(current) = open_set.pop() {
        if current.point.x == goal.x && current.point.y == goal.y {
            return Some(current.steps);
        }

        if !closed_set.insert(current.point) {
            continue;
        }

        for (dx, dy) in directions.iter() {
            let next = Point {
                x: current.point.x + dx,
                y: current.point.y + dy,
            };

            if !is_valid_point(&next, size) || closed_set.contains(&next) || dangers.contains(&next)
            {
                continue;
            }

            let next_node = Node {
                point: next,
                steps: current.steps + 1,
                f_score: (current.steps + 1) + manhattan_distance(&next, &goal),
            };

            open_set.push(next_node);
        }
    }
    None
}

fn main() -> io::Result<()> {
    let danger_points = read_danger_points()?;
    let grid_size = 70; // This now means 0-70 inclusive

    match find_shortest_path(grid_size, danger_points) {
        Some(steps) => println!("Shortest path found: {} steps", steps),
        None => println!("No valid path found!"),
    }

    Ok(())
}