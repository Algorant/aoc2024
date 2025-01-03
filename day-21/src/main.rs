use std::collections::HashMap;
use std::io::{self, Read};

const NUMERIC: [&str; 4] = ["789", "456", "123", " 0A"];
const DIRECTIONAL: [&str; 3] = [" ^A", "<v>", " vA"];

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

fn find_char(keypad: &[&str], target: char) -> Pos {
    for (y, row) in keypad.iter().enumerate() {
        if let Some(x) = row.chars().position(|c| c == target) {
            return Pos {
                x: x as i32,
                y: y as i32,
            };
        }
    }
    panic!("Character not found in keypad");
}

fn walk(keypad: &[&str], start: Pos, path: &str) -> Vec<char> {
    let mut result = Vec::new();
    let mut current = start;

    for direction in path.chars() {
        let (dx, dy) = match direction {
            '<' => (-1, 0),
            '>' => (1, 0),
            '^' => (0, -1),
            'v' => (0, 1),
            _ => panic!("Invalid direction"),
        };

        current.x += dx;
        current.y += dy;

        if current.y >= 0 && (current.y as usize) < keypad.len() {
            let row = keypad[current.y as usize];
            if current.x >= 0 && (current.x as usize) < row.len() {
                result.push(row.chars().nth(current.x as usize).unwrap());
            }
        }
    }
    result
}

fn paths_between(keypad: &[&str], start: char, end: char) -> Vec<String> {
    let start_pos = find_char(keypad, start);
    let end_pos = find_char(keypad, end);

    let dx = end_pos.x - start_pos.x;
    let dy = end_pos.y - start_pos.y;

    let mut paths = Vec::new();

    let hor = if dx > 0 {
        ">".repeat(dx.abs() as usize)
    } else {
        "<".repeat(dx.abs() as usize)
    };

    let ver = if dy > 0 {
        "v".repeat(dy.abs() as usize)
    } else {
        "^".repeat(dy.abs() as usize)
    };

    // Try both horizontal then vertical and vertical then horizontal
    let path1 = format!("{}{}", hor, ver);
    let path2 = format!("{}{}", ver, hor);

    for path in [path1, path2].iter() {
        if !walk(keypad, start_pos, path).contains(&' ') {
            paths.push(format!("{}A", path));
        }
    }
    paths
}

fn cost_between(
    cache: &mut HashMap<(char, char, usize), i64>,
    keypad: &[&str],
    start: char,
    end: char,
    links: usize,
) -> i64 {
    if links == 0 {
        return 1;
    }

    let key = (start, end, links);
    if let Some(&cost) = cache.get(&key) {
        return cost;
    }

    let paths = paths_between(keypad, start, end);
    let min_cost = paths
        .iter()
        .map(|path| cost(cache, &DIRECTIONAL, path, links - 1))
        .min()
        .unwrap_or(i64::MAX);

    cache.insert(key, min_cost);
    min_cost
}

fn cost(
    cache: &mut HashMap<(char, char, usize), i64>,
    keypad: &[&str],
    keys: &str,
    links: usize,
) -> i64 {
    let mut total = 0;
    let full_keys = format!("A{}", keys);
    let chars: Vec<char> = full_keys.chars().collect();

    for i in 0..chars.len() - 1 {
        total += cost_between(cache, keypad, chars[i], chars[i + 1], links);
    }
    total
}

fn complexity(code: &str, robots: usize) -> i64 {
    let mut cache = HashMap::new();
    let numeric_value: i64 = code[..code.len() - 1].parse().unwrap();
    cost(&mut cache, &NUMERIC, code, robots + 1) * numeric_value
}

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let codes: Vec<&str> = input.split_whitespace().collect();

    let part1: i64 = codes.iter().map(|&code| complexity(code, 2)).sum();
    let part2: i64 = codes.iter().map(|&code| complexity(code, 25)).sum();

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}
