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

fn can_make_target_part1(numbers: &[i64], target: i64) -> bool {
    // For n numbers, we need n-1 operators
    let num_ops = numbers.len() - 1;
    let max_combinations = 1 << num_ops; // 2^(n-1) possible combinations

    // Try each possible combo
    for combo in 0..max_combinations {
        let mut result = numbers[0];

        // Use bits of combo to determine operators (0 = add, 1 = multiply)
        for i in 0..num_ops {
            let next_num = numbers[i + 1];
            if (combo >> i) & 1 == 0 {
                result += next_num;
            } else {
                result *= next_num;
            }
        }

        if result == target {
            return true;
        }
    }
    false
}

fn can_make_target_part2(numbers: &[i64], target: i64) -> bool {
    // For n numbers, we need n-1 operators
    let num_ops = numbers.len() - 1;
    let max_combinations = 1 << (2 * num_ops); // 3^(n-1) possible combinations

    // Try each possible combo
    for combo in 0..max_combinations {
        let mut result = numbers[0];

        // Use pairs of bits to determine operators (00 = add, 01 = multiply, 10 = concatenate)
        for i in 0..num_ops {
            let op = (combo >> (2 * i)) & 3; // Get 2 bits for this operator
            let next_num = numbers[i + 1];

            match op {
                0 => result += next_num, // Addition
                1 => result *= next_num, // Multiplication
                2 => {
                    // Concatenation
                    let concat = format!("{}{}", result, next_num);
                    if let Ok(concat_num) = concat.parse::<i64>() {
                        result = concat_num;
                    } else {
                        // If concatenation would overflow, this combination is invalid
                        break;
                    }
                }
                _ => break, // Invalid operator, skip this combination
            }
        }

        if result == target {
            return true;
        }
    }
    false
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
