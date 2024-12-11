use std::collections::HashMap;
use std::fs::read_to_string;

fn blink_loop(number_counts: &mut HashMap<u64, usize>) {
    let mut new_counts = HashMap::new();

    for (&num, &count) in number_counts.iter() {
        if num == 0 {
            // Rule 1: 0 becomes 1
            *new_counts.entry(1).or_insert(0) += count;
        } else if num.to_string().len() % 2 == 0 {
            // Rule 2: Split even digit numbers
            let num_str = num.to_string();
            let half_len = num_str.len() / 2;
            let first_half = num_str[..half_len].parse::<u64>().unwrap();
            let second_half = num_str[half_len..].parse::<u64>().unwrap();

            *new_counts.entry(first_half).or_insert(0) += count;
            *new_counts.entry(second_half).or_insert(0) += count;
        } else {
            // Rule 3: Multiply by 2024
            *new_counts.entry(num * 2024).or_insert(0) += count;
        }
    }

    *number_counts = new_counts;
}

fn apply_blinks(input: &str, blinks: usize) -> usize {
    let mut number_counts: HashMap<u64, usize> = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut acc, num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

    for i in 0..blinks {
        blink_loop(&mut number_counts);
        if i % 5 == 0 {
            println!("After {} blinks: {} unique numbers", i, number_counts.len());
        }
    }

    // Return total count of numbers
    number_counts.values().sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!(
        "Part 1 - Length after 25 blinks: {}",
        apply_blinks(input.trim(), 25)
    );
    println!(
        "Part 2 - Length after 75 blinks: {}",
        apply_blinks(input.trim(), 75)
    );
}
