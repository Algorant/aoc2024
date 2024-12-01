use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    let lines: Vec<&str> = input.lines().collect();

    // Print first line of input
    println!("First line of input: '{}'", lines[0]);

    if lines.len() != 1000 {
        panic!("Expected 1000 lines, found {}", lines.len());
    }

    let l1: Vec<u32> = lines
        .iter()
        .map(|line| {
            let nums: Vec<&str> = line.split_whitespace().collect();
            nums[0].parse().expect("Failed to parse first number")
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sorted()
        .collect();

    let l2: Vec<u32> = lines
        .iter()
        .map(|line| {
            let nums: Vec<&str> = line.split_whitespace().collect();
            nums[1].parse().expect("Failed to parse second number")
        })
        .collect::<Vec<u32>>()
        .into_iter()
        .sorted()
        .collect();

    // Get difference between l1 and l2 per line and sum them up
    let difflist: Vec<u32> = l1
        .iter()
        .zip(l2.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .collect();

    let sumlist: u32 = difflist.iter().sum();
    println!("Sum of differences: {}", sumlist);

    // Part 2
    let similarity: Vec<u32> = l1
        .iter()
        .map(|num| {
            let count = l2.iter().filter(|&x| x == num).count();
            num * (count as u32)
        })
        .collect();

    let result: u32 = similarity.iter().sum();
    println!("Sum of similarities: {}", result);
}
