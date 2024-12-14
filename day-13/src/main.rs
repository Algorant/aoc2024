// Input has
// Button A: X+int, Y+int
// Button B: X+int, Y+int
// Prize: X=int, Y=int

#[derive(Debug)]
struct Button {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Prize {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> (Button, Button, Prize) {
    let lines: Vec<&str> = input.lines().collect();

    // Parse A Button
    let a_parts: Vec<&str> = lines[0].split(": ").nth(1).unwrap().split(", ").collect();
    let a_x: i64 = a_parts[0].trim_start_matches("X+").parse::<i64>().unwrap();
    let a_y: i64 = a_parts[1].trim_start_matches("Y+").parse::<i64>().unwrap();

    // Parse B Button
    let b_parts: Vec<&str> = lines[1].split(": ").nth(1).unwrap().split(", ").collect();
    let b_x: i64 = b_parts[0].trim_start_matches("X+").parse::<i64>().unwrap();
    let b_y: i64 = b_parts[1].trim_start_matches("Y+").parse::<i64>().unwrap();

    // Parse Prize
    let prize_parts: Vec<&str> = lines[2].split(": ").nth(1).unwrap().split(", ").collect();
    let p_x: i64 = prize_parts[0]
        .trim_start_matches("X=")
        .parse::<i64>()
        .unwrap();
    let p_y: i64 = prize_parts[1]
        .trim_start_matches("Y=")
        .parse::<i64>()
        .unwrap();

    (
        Button { x: a_x, y: a_y },
        Button { x: b_x, y: b_y },
        Prize { x: p_x, y: p_y },
    )
}

fn parse_machines(content: &str) -> Vec<(Button, Button, Prize)> {
    let mut machines = Vec::new();
    let mut current_lines = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            if !current_lines.is_empty() {
                let machine_input = current_lines.join("\n");
                machines.push(parse_input(&machine_input));
                current_lines.clear();
            }
        } else {
            current_lines.push(line);
        }
    }

    // Don't forget the last machine if file doesn't end with empty line
    if !current_lines.is_empty() {
        let machine_input = current_lines.join("\n");
        machines.push(parse_input(&machine_input));
    }

    machines
}

// Part 2: Implement Cramer's Rule
fn solve_with_cramers_rule(
    button_a: &Button,
    button_b: &Button,
    prize: &Prize,
) -> Option<(i64, i64)> {
    // Use Cramer's Rule to solve the system of linear equations:
    // button_a.x * A + button_b.x * B = prize.x
    // button_a.y * A + button_b.y * B = prize.y

    // Calculate the determinant of the coefficient matrix
    let det = (button_a.x * button_b.y) - (button_a.y * button_b.x);

    if det == 0 {
        return None; // No unique solution exists
    }

    // Calculate determinants for A and B
    let det_a = (prize.x * button_b.y) - (prize.y * button_b.x);
    let det_b = (button_a.x * prize.y) - (button_a.y * prize.x);

    // Calculate A and B (number of presses needed)
    let a = det_a as f64 / det as f64;
    let b = det_b as f64 / det as f64;

    // Check if we have positive integer solutions
    if a >= 0.0 && b >= 0.0 && a.fract() == 0.0 && b.fract() == 0.0 {
        return Some((a as i64, b as i64));
    }

    None
}

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let machines = parse_machines(&content);

    // Token costs can be easily modified here
    let button_a_cost: i64 = 3;
    let button_b_cost: i64 = 1;

    // Part 1
    let mut total_tokens_p1 = 0;
    let mut impossible_count_p1 = 0;

    println!("Part 1:");
    println!("---------");
    for (i, (button_a, button_b, prize)) in machines.iter().enumerate() {
        println!("Machine {}:", i + 1);
        match solve_with_cramers_rule(button_a, button_b, prize) {
            Some((a_presses, b_presses)) => {
                let machine_tokens = (a_presses * button_a_cost) + (b_presses * button_b_cost);
                println!("  Solution found!");
                println!(
                    "  Button A presses: {} (cost: {})",
                    a_presses,
                    a_presses * button_a_cost
                );
                println!(
                    "  Button B presses: {} (cost: {})",
                    b_presses,
                    b_presses * button_b_cost
                );
                println!("  Total tokens needed: {}", machine_tokens);
                total_tokens_p1 += machine_tokens;
            }
            None => {
                println!("  No solution found");
                impossible_count_p1 += 1;
            }
        }
        println!();
    }

    println!("Part 1 Summary:");
    println!(
        "Total tokens needed for all possible machines: {}",
        total_tokens_p1
    );
    println!("Number of impossible machines: {}", impossible_count_p1);
    println!();

    // Part 2
    let offset: i64 = 10000000000000;
    let mut total_tokens_p2 = 0;
    let mut impossible_count_p2 = 0;

    println!("Part 2:");
    println!("---------");
    for (i, (button_a, button_b, prize)) in machines.iter().enumerate() {
        println!("Machine {}:", i + 1);
        // Create modified prize with offset
        let modified_prize = Prize {
            x: prize.x + offset,
            y: prize.y + offset,
        };

        match solve_with_cramers_rule(button_a, button_b, &modified_prize) {
            Some((a_presses, b_presses)) => {
                let machine_tokens = (a_presses * button_a_cost) + (b_presses * button_b_cost);
                println!("  Solution found!");
                println!(
                    "  Button A presses: {} (cost: {})",
                    a_presses,
                    a_presses * button_a_cost
                );
                println!(
                    "  Button B presses: {} (cost: {})",
                    b_presses,
                    b_presses * button_b_cost
                );
                println!("  Total tokens needed: {}", machine_tokens);
                total_tokens_p2 += machine_tokens;
            }
            None => {
                println!("  No solution found");
                impossible_count_p2 += 1;
            }
        }
        println!();
    }

    println!("Part 2 Summary:");
    println!(
        "Total tokens needed for all possible machines: {}",
        total_tokens_p2
    );
    println!("Number of impossible machines: {}", impossible_count_p2);
}
