use std::fs::read_to_string;

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    // Split along the : to get two sides of the input
    let parts: Vec<&str> = line.split(':').collect();
    // The target number
    let target = parts[0].trim().parse().unwrap();
    // The list of possible operator combination numbers
    let numbers: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();
    (target, numbers)
}

fn try_combos_part1(numbers: &[i64], target: i64, index: usize, current_result: i64) -> bool {
    // Base case: if we've used all numbers, check if we've reached the target
    if index == numbers.len() {
        return current_result == target;
    }

    // If this is first number, start with it
    if index == 0 {
        return try_combos_part1(numbers, target, index + 1, numbers[0]);
    }

    // Try addition
    if try_combos_part1(numbers, target, index + 1, current_result + numbers[index]) {
        return true;
    }

    // Try multiplication
    try_combos_part1(numbers, target, index + 1, current_result * numbers[index])
}

fn try_combos_part2(numbers: &[i64], target: i64, index: usize, current_result: i64) -> bool {
    // Base case: if we've used all numbers, check if we've reached the target
    if index == numbers.len() {
        return current_result == target;
    }

    // If this is first number, start with it
    if index == 0 {
        return try_combos_part2(numbers, target, index + 1, numbers[0]);
    }

    // Try addition
    if try_combos_part2(numbers, target, index + 1, current_result + numbers[index]) {
        return true;
    }

    // Try multiplication
    if try_combos_part2(numbers, target, index + 1, current_result * numbers[index]) {
        return true;
    }

    // Try concatenation
    let concat = format!("{}{}", current_result, numbers[index]);
    if let Ok(concat_num) = concat.parse::<i64>() {
        if try_combos_part2(numbers, target, index + 1, concat_num) {
            return true;
        }
    }
    false
}

fn can_make_target_part1(numbers: &[i64], target: i64) -> bool {
    try_combos_part1(numbers, target, 0, 0)
}

fn can_make_target_part2(numbers: &[i64], target: i64) -> bool {
    try_combos_part2(numbers, target, 0, 0)
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // Keep track of progress for part 1 (+ and * only)
    let mut possible_count_p1 = 0;
    let mut target_sum_p1 = 0;

    // Keep track of progress for additional matches from part 2 (concatenation)
    let mut additional_count = 0;
    let mut additional_sum = 0;

    for line in input.lines() {
        let (target, numbers) = parse_line(line);
        let possible_p1 = can_make_target_part1(&numbers, target);

        if possible_p1 {
            possible_count_p1 += 1;
            target_sum_p1 += target;
            println!(
                "Target: {}, Numbers: {:?}, Possible with +/* only",
                target, numbers
            );
        } else {
            // Only try part 2 logic if part 1 failed
            let possible_p2 = can_make_target_part2(&numbers, target);
            if possible_p2 {
                additional_count += 1;
                additional_sum += target;
                println!(
                    "Target: {}, Numbers: {:?}, Possible with concatenation",
                    target, numbers
                );
            } else {
                println!("Target: {}, Numbers: {:?}, Not possible", target, numbers);
            }
        }
    }

    println!("\nPart 1 Results (+ and * only):");
    println!("Number of possible combinations: {}", possible_count_p1);
    println!("Sum of possible targets: {}", target_sum_p1);

    println!("\nAdditional Results (concatenation only):");
    println!("Number of additional possibilities: {}", additional_count);
    println!("Sum of additional targets: {}", additional_sum);

    println!("\nTotal Results:");
    println!(
        "Total possible combinations: {}",
        possible_count_p1 + additional_count
    );
    println!("Total sum of targets: {}", target_sum_p1 + additional_sum);
}
