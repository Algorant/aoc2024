use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

fn read_input() -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);
    let mut patterns = Vec::new();
    let mut designs = Vec::new();
    let mut reading_patterns = true;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            reading_patterns = false;
            continue;
        }

        if reading_patterns {
            // Split patterns by comma and trim whitespace
            patterns.extend(
                line.split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty()),
            );
        } else {
            designs.push(line);
        }
    }

    Ok((patterns, designs))
}

fn count_design_permutations(design: &str, patterns: &[String]) -> usize {
    fn recursive_count(
        remaining: &str,
        patterns: &[String],
        memo: &mut HashMap<String, usize>,
    ) -> usize {
        if remaining.is_empty() {
            return 1;
        }

        // Check memoized result
        if let Some(&count) = memo.get(remaining) {
            return count;
        }

        let mut total = 0;
        // Try all patterns at the current position
        for pattern in patterns.iter() {
            if remaining.starts_with(pattern) {
                total += recursive_count(&remaining[pattern.len()..], patterns, memo);
            }
        }

        memo.insert(remaining.to_string(), total);
        total
    }

    let mut memo = HashMap::new();
    recursive_count(design, patterns, &mut memo)
}

fn main() -> io::Result<()> {
    let (patterns, designs) = read_input()?;

    // Don't filter or sort patterns, we need all of them in their original form
    println!("Patterns loaded: {}", patterns.len());
    println!("Designs loaded: {}", designs.len());

    println!("Patterns loaded: {}", patterns.len());
    println!("Designs loaded: {}", designs.len());

    // Part 1: Count valid designs
    let valid_count = designs
        .iter()
        .filter(|design| count_design_permutations(design, &patterns) > 0)
        .count();
    println!("Part 1 - Number of valid designs: {}", valid_count);

    // Part 2: Sum of all possible permutations
    let total_permutations: usize = designs
        .iter()
        .map(|design| count_design_permutations(design, &patterns))
        .sum();
    println!(
        "Part 2 - Total number of permutations: {}",
        total_permutations
    );

    Ok(())
}

//fn main() -> io::Result<()> {
//    let (patterns, designs) = read_input()?;
//
//    println!("Patterns loaded: {}", patterns.len());
//    println!("Designs loaded: {}", designs.len());
//
//    // Part 1: Count valid designs
//    let valid_count = designs
//        .iter()
//        .filter(|design| count_design_permutations(design, &patterns) > 0)
//        .count();
//    println!("Part 1 - Number of valid designs: {}", valid_count);
//
//    // Part 2: Sum of all possible permutations
//    let total_permutations: usize = designs
//        .iter()
//        .map(|design| count_design_permutations(design, &patterns))
//        .sum();
//    println!(
//        "Part 2 - Total number of permutations: {}",
//        total_permutations
//    );
//
//    Ok(())
//}
