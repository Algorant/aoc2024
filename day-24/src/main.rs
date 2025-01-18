use std::collections::HashMap;

#[derive(Debug)]
enum Operation {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<(Operation, String)>) {
    let mut circuits = HashMap::new();
    let mut operations = Vec::new();
    let mut parsing_operations = false;

    for line in input.lines() {
        if line.is_empty() {
            parsing_operations = true;
            continue;
        }

        if !parsing_operations {
            // Parse initial values
            let parts: Vec<&str> = line.split(": ").collect();
            let circuit = parts[0].to_string();
            let value = parts[1].trim() == "1";
            circuits.insert(circuit, value);
        } else {
            // Parse operations
            let parts: Vec<&str> = line.split(" -> ").collect();
            let operation_parts: Vec<&str> = parts[0].split_whitespace().collect();

            let operation = match operation_parts[1] {
                "AND" => Operation::And(
                    operation_parts[0].to_string(),
                    operation_parts[2].to_string(),
                ),
                "OR" => Operation::Or(
                    operation_parts[0].to_string(),
                    operation_parts[2].to_string(),
                ),
                "XOR" => Operation::Xor(
                    operation_parts[0].to_string(),
                    operation_parts[2].to_string(),
                ),
                _ => panic!("Unknown operation"),
            };

            operations.push((operation, parts[1].trim().to_string()));
        }
    }
    // Debug print
    //println!("\nParsed initial circuits:");
    //for (k, v) in &circuits {
    //    println!("{}: {}", k, if *v { 1 } else { 0 });
    //}
    //
    //println!("\nParsed operations:");
    //for (op, target) in &operations {
    //    println!("{:?} -> {}", op, target);
    //}

    (circuits, operations)
}

fn evaluate_circuits(
    mut circuits: HashMap<String, bool>,
    operations: &[(Operation, String)],
) -> HashMap<String, bool> {
    let mut iteration = 0;
    let max_iterations = operations.len() * 2; // Safety limit

    loop {
        let mut changes_made = false;
        iteration += 1;

        // Create a snapshot of current state for comparison
        for (operation, target) in operations {
            let (input1, input2) = match operation {
                Operation::And(a, b) | Operation::Or(a, b) | Operation::Xor(a, b) => (a, b),
            };

            // Get current values of inputs (they might have changed)
            if let (Some(&value1), Some(&value2)) = (circuits.get(input1), circuits.get(input2)) {
                let new_result = match operation {
                    Operation::And(_, _) => value1 & value2,
                    Operation::Or(_, _) => value1 | value2,
                    Operation::Xor(_, _) => value1 ^ value2,
                };

                // Update or insert the new result
                if let Some(&old_value) = circuits.get(target) {
                    if old_value != new_result {
                        circuits.insert(target.clone(), new_result);
                        changes_made = true;
                        println!(
                            "Updated {}: {} -> {}",
                            target, old_value as u8, new_result as u8
                        );
                    }
                } else {
                    circuits.insert(target.clone(), new_result);
                    changes_made = true;
                    //println!("New circuit {}: {}", target, new_result as u8);
                }
            }
        }

        // Check if we've reached a stable state
        if !changes_made || iteration > max_iterations {
            //println!("Evaluation completed after {} iterations", iteration);
            break;
        }
    }

    circuits
}

fn main() {
    // Read input file
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    // Parse input into circuits and operations
    let (initial_circuits, operations) = parse_input(&input);

    // Evaluate all operations
    let final_circuits = evaluate_circuits(initial_circuits, &operations);

    // Print results
    //println!("Final circuit states:");
    let mut sorted_circuits: Vec<_> = final_circuits.iter().collect();
    sorted_circuits.sort_by(|a, b| a.0.cmp(b.0));

    //for (circuit, value) in &sorted_circuits {
    //    println!("{}: {}", circuit, if **value { 1 } else { 0 });
    //}

    // Extract the z-circuits and build binary number
    let mut z_circuits: Vec<_> = sorted_circuits
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();

    // Sort by the number after 'z' to ensure correct order
    z_circuits.sort_by(|(a, _), (b, _)| {
        let a_num = a[1..].parse::<usize>().unwrap();
        let b_num = b[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });

    // Build binary string and decimal value
    let binary: String = z_circuits
        .iter()
        .rev()
        .map(|(_, &value)| if value { '1' } else { '0' })
        .collect();

    let decimal = usize::from_str_radix(&binary, 2).unwrap();

    println!("\nZ-circuit output:");
    println!("Binary:  {}", binary);
    println!("Decimal: {}", decimal);
}
