use std::fs::read_to_string;

// Helper function to filter the various requirements
fn analyze_sequence(line: &str) -> &'static str {
    let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let mut increasing = true;
    let mut decreasing = true;
    let mut valid_report = true;

    for i in 1..numbers.len() {
        let diff = if numbers[i] > numbers[i - 1] {
            numbers[i] - numbers[i - 1]
        } else {
            numbers[i - 1] - numbers[i]
        };

        // Check for valid incrementation

        if diff == 0 || diff > 3 {
            valid_report = false;
            break;
        }

        if numbers[i] <= numbers[i - 1] {
            increasing = false;
        }

        if numbers[i] >= numbers[i - 1] {
            decreasing = false;
        }
    }

    if !valid_report {
        "neither"
    } else if increasing {
        "increasing"
    } else if decreasing {
        "decreasing"
    } else {
        "neither"
    }
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.lines().collect();

    // Print first line of input
    println!("First line of input: '{}'", lines[0]);

    if lines.len() != 1000 {
        panic!("Expected 1000 lines, found {}", lines.len());
    }

    // Use helper to check if increasing or decreasing
    let mut increasing_count = 0;
    let mut decreasing_count = 0;
    let mut neither_count = 0;

    for line in lines {
        match analyze_sequence(line) {
            "increasing" => increasing_count += 1,
            "decreasing" => decreasing_count += 1,
            "neither" => neither_count += 1,
            _ => unreachable!(),
        }
    }
    println!("Increasing Sequences: {}", increasing_count);
    println!("Decreasing Sequences: {}", decreasing_count);
    println!("Total Valid Tests {}", increasing_count + decreasing_count);
    println!("Disqualified Sequences: {}", neither_count);
}
