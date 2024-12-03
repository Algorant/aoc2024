use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");

    // Part 2
    // First pass: Create map of do and don'ts, track state with a boolean
    let do_re = Regex::new(r"(don't\(\)|do\(\))").unwrap();
    let mut state_changes: Vec<(usize, bool)> = vec![(0, true)]; // Start with do aka true or "on"

    for cap in do_re.captures_iter(&input) {
        let pos = cap.get(0).unwrap().start();
        let is_do = &cap[0] == "do()";
        state_changes.push((pos, is_do));
    }
    state_changes.sort_by_key(|k| k.0);

    println!("State Changes:");
    for (pos, state) in &state_changes {
        println!(
            "Position {}: switching to {}",
            pos,
            if *state { "do() e" } else { "don't()" }
        );
    }
    println!("----------------------------------------\n");

    // Second pass: Find the mul patterns and check if they are within do or don't areas
    //let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})").unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum: i32 = 0;

    for cap in mul_re.captures_iter(&input) {
        let mul_pos = cap.get(0).unwrap().start();

        // Find last state change before this position
        let current_state = state_changes
            .iter()
            .rev()
            .find(|(pos, _)| *pos <= mul_pos)
            .map(|(_, is_do)| *is_do)
            .unwrap_or(true);

        if current_state {
            let n1: i32 = cap[1].parse().unwrap();
            let n2: i32 = cap[2].parse().unwrap();
            let result = n1 * n2;
            println!(
                "Counted: mul({},{}) = {} at position {}",
                n1, n2, result, mul_pos
            );
            sum += result;
        } else {
            let n1: i32 = cap[1].parse().unwrap();
            let n2: i32 = cap[2].parse().unwrap();
            println!("Skipped: mul({},{}) at position {}", n1, n2, mul_pos);
        }
    }

    println!("\nSum of valid multiplications: {}", sum);

    // Part 1

    // Create regex pattern
    //let old_re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    //let new_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Find all matches and print contents (for debugging and to make sure its correct)
    //for cap in old_re.captures_iter(&input) {
    //    println!("{}", &cap[0]);
    //}

    // Calculate sum of all multiplications
    //let sum: i32 = new_re
    //    .captures_iter(&input)
    //    .map(|cap| {
    //        // Extract each integer
    //        let n1: i32 = cap[1].parse().unwrap();
    //        let n2: i32 = cap[2].parse().unwrap();
    //
    //        // Multiply
    //        let result = n1 * n2;
    //        println!("{}*{} = {}", n1, n2, result);
    //
    //        result
    //    })
    //    .sum();
    //
    //println!("Sum of all multiplications: {}", sum);
}
