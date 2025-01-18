use std::fs::read_to_string;

#[derive(Debug)]
struct Pattern {
    pattern_type: PatternType,
    heights: Vec<u32>,
}

#[derive(Debug)]
enum PatternType {
    Lock,
    Key,
}

fn parse_attern(lines: &[&str]) -> Option<Pattern> {
    if lines.len() != 7 {
        return None;
    }

    let is_lock = lines[0].chars().all(|c| c == '#') && lines[6].chars().all(|c| c == '.');
    let is_key = lines[0].chars().all(|c| c == '.') && lines[6].chars().all(|c| c == '#');

    if !is_lock && !is_key {
        return None;
    }

    let mut heights = vec![0; 5];

    // Count # in each column for the middle 5 rows (index 1 to 5)
    for col in 0..5 {
        for row in 1..6 {
            if lines[row].chars().nth(col) == Some('#') {
                heights[col] += 1;
            }
        }
    }

    Some(Pattern {
        pattern_type: if is_lock {
            PatternType::Lock
        } else {
            PatternType::Key
        },
        heights,
    })
}

fn parse_input(content: &str) -> Vec<Pattern> {
    let lines: Vec<&str> = content.lines().collect();
    let mut patterns = Vec::new();
    let mut i = 0;

    while i + 7 <= lines.len() {
        let current_lines = &lines[i..i + 7];
        if let Some(pattern) = parse_attern(current_lines) {
            patterns.push(pattern);
            i += 8; // Skip the empty line between patterns
        } else {
            i += 1;
        }
    }
    patterns
}

fn do_patterns_overlap(lock: &Pattern, key: &Pattern) -> bool {
    lock.heights
        .iter()
        .zip(key.heights.iter())
        .any(|(&l, &k)| l + k > 5)
}

fn count_non_overlapping_pairs(locks: &[&Pattern], keys: &[&Pattern]) -> usize {
    let mut count = 0;

    for lock in locks {
        for key in keys {
            if !do_patterns_overlap(lock, key) {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let content = read_to_string("input.txt").unwrap();
    let patterns = parse_input(&content);

    let (locks, keys): (Vec<_>, Vec<_>) = patterns
        .iter()
        .partition(|p| matches!(p.pattern_type, PatternType::Lock));

    println!("\nNumber of locks: {}", locks.len());
    println!("Number of keys: {}", keys.len());

    let pairs = count_non_overlapping_pairs(&locks, &keys);
    println!("\nNumber of unique lock and key pairs: {}", pairs);
}
