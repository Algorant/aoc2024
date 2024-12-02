use std::fs::read_to_string;

// Helper function to filter the various requirements
fn analyze_sequence(numbers: &[u32]) -> &'static str {
    //let numbers: Vec<u32> = line
    //    .split_whitespace()
    //    .map(|num_str| num_str.parse().unwrap())
    //    .collect();

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

fn analyze_sequence_str(line: &str) -> (&'static str, Option<String>) {
    let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    let result = analyze_sequence(&numbers);
    if result == "neither" {
        (result, Some(line.to_string()))
    } else {
        (result, None)
    }
}

fn try_fix_sequence(sequence: &str) -> Option<(usize, &'static str)> {
    let numbers: Vec<u32> = sequence
        .split_whitespace()
        .map(|num_str| num_str.parse().unwrap())
        .collect();

    // Try removing each number and analyze the resulting sequence
    for i in 0..numbers.len() {
        let mut test_numbers = numbers.clone();
        test_numbers.remove(i);

        let result = analyze_sequence(&test_numbers);
        if result != "neither" {
            return Some((i, result));
        }
    }
    None
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
    // Keep track of disqualified sequences
    let mut disqualified_sequences: Vec<String> = Vec::new();

    for line in lines {
        match analyze_sequence_str(line) {
            ("increasing", _) => increasing_count += 1,
            ("decreasing", _) => decreasing_count += 1,
            ("neither", Some(seq)) => {
                neither_count += 1;
                disqualified_sequences.push(seq);
            }
            _ => unreachable!(),
        }
    }
    println!("Initial Analysis:");
    println!("Increasing Sequences: {}", increasing_count);
    println!("Decreasing Sequences: {}", decreasing_count);
    println!("Total Valid Tests {}", increasing_count + decreasing_count);
    println!("Disqualified Sequences: {}", neither_count);

    // Process disqualified sequences for Part 2

    println!("\n==========================================");
    println!(" Begin Part 2 Analysis");
    println!("==========================================");
    let mut recovered_sequences = 0;
    let mut recovered_increasing = 0;
    let mut recovered_decreasing = 0;

    for (index, seq) in disqualified_sequences.iter().enumerate() {
        if let Some((problem_index, new_result)) = try_fix_sequence(seq) {
            recovered_sequences += 1;
            match new_result {
                "increasing" => recovered_increasing += 1,
                "decreasing" => recovered_decreasing += 1,
                _ => unreachable!(),
            }

            let numbers: Vec<&str> = seq.split_whitespace().collect();
            println!(
                "Sequence {} can be fixed by removing number {} at position {}: Result: {}",
                index + 1,
                numbers[problem_index],
                problem_index + 1,
                new_result
            );
        }
    }
    println!("\nRecovery Results:");
    println!("Total Recovered Sequences: {}", recovered_sequences);
    println!("Recovered Increasing: {}", recovered_increasing);
    println!("Recovered Decreasing: {}", recovered_decreasing);
    println!(
        "Final Invalid Sequences: {}",
        neither_count - recovered_sequences
    );
    println!(
        "Total Valid Sequences after Part 2: {}",
        1000 - (neither_count - recovered_sequences)
    );
}
