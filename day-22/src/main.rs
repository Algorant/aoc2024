use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_secret(mut secret: u64, sequences: u32) -> u64 {
    for _ in 0..sequences {
        // Multiply by 64, mix, and prune
        let multiplied = secret * 64;
        secret ^= multiplied;
        secret %= 16777216;

        // Step 2: Divide by 32, mix, and prune
        let divided = secret / 32;
        secret ^= divided;
        secret %= 16777216;

        // Step 3: Multiply by 2048, mix, and prune
        let multiplied_final = secret * 2048;
        secret ^= multiplied_final;
        secret %= 16777216;
    }

    secret
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let numbers: Vec<u64> = input.lines().filter_map(|line| line.parse().ok()).collect();

    //let sample_numbers = vec![1, 10, 100, 2024];

    let num_sequences = 2000;
    let mut sum = 0;

    for secret_number in numbers {
        let result = process_secret(secret_number, num_sequences);
        println!("Initial secret: {}", secret_number);
        println!("After {} sequences: {}", num_sequences, result);
        sum += result;
    }
    println!("Sum of results: {}", sum);
}
