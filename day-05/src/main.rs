use std::fs::read_to_string;

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl Rule {
    fn new(line: &str) -> Result<Self, String> {
        //println!("Trying to parse rule: '{}'", line);
        let parts: Vec<&str> = line.split('|').collect();

        if parts.len() != 2 {
            return Err(format!(
                "Expected 2 parts, found {}: '{}'",
                parts.len(),
                line
            ));
        }

        let first = parts[0]
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse first number '{}': {}", parts[0], e))?;

        let second = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("Failed to parse second number '{}': {}", parts[1], e))?;

        Ok(Rule { first, second })
    }
}

#[derive(Debug)]
struct ValidationResult {
    middle_number: u32,
    fixed_sequence: Option<Vec<u32>>,
}

fn fix_sequence(nums: &[u32], rules: &[Rule]) -> Option<Vec<u32>> {
    let mut sequence = nums.to_vec();
    //let len = sequence.len();

    // Sequence fixing logic for Part 2
    loop {
        let mut made_swap = false;

        for rule in rules {
            if let (Some(first_pos), Some(second_pos)) = (
                sequence.iter().position(|&x| x == rule.first),
                sequence.iter().position(|&x| x == rule.second),
            ) {
                if first_pos >= second_pos {
                    // Rule is violated, swap adjacent elements to fix it
                    for i in (second_pos..first_pos).rev() {
                        sequence.swap(i, i + 1);
                        made_swap = true;
                    }
                }
            }
        }

        // If no swap needed, sequence valid
        if !made_swap {
            break;
        }
    }

    // Verify sequence is now valid
    if rules.iter().all(|rule| {
        if let (Some(first_pos), Some(second_pos)) = (
            sequence.iter().position(|&x| x == rule.first),
            sequence.iter().position(|&x| x == rule.second),
        ) {
            first_pos < second_pos
        } else {
            true
        }
    }) {
        Some(sequence)
    } else {
        None
    }
}

fn validate_sequence(sequence: &str, rules: &[Rule]) -> ValidationResult {
    // First parse sequence numbers
    let nums: Vec<u32> = sequence
        .split(',')
        .map(|s| s.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    let middle_number = nums[nums.len() / 2];

    let is_valid = rules.iter().all(|rule| {
        if let (Some(first_pos), Some(second_pos)) = (
            nums.iter().position(|&x| x == rule.first),
            nums.iter().position(|&x| x == rule.second),
        ) {
            first_pos < second_pos
        } else {
            true
        }
    });

    if is_valid {
        return ValidationResult {
            middle_number,
            fixed_sequence: None,
        };
    }

    // Try to fix
    if let Some(fixed_nums) = fix_sequence(&nums, rules) {
        ValidationResult {
            middle_number: fixed_nums[fixed_nums.len() / 2],
            fixed_sequence: Some(fixed_nums),
        }
    } else {
        ValidationResult {
            middle_number: nums[nums.len() / 2],
            fixed_sequence: None,
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (rules_str, sequences) = input
        .split_once("\n\n")
        .expect("Failed to split input at empty line");

    // Parse rules with error handling
    let rules: Vec<Rule> = rules_str
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| match Rule::new(line) {
            Ok(rule) => Some(rule),
            Err(e) => {
                println!("Error parsing rule: {}", e);
                None
            }
        })
        .collect();

    let sequences: Vec<&str> = sequences.lines().filter(|line| !line.is_empty()).collect();

    println!("Successfully parsed {} rules", rules.len());

    // First pass: originally valid sequences
    let mut valid_count = 0;
    let mut original_sum = 0;
    let mut sequences_to_fix = Vec::new();

    for (i, sequence) in sequences.iter().enumerate() {
        let nums: Vec<u32> = sequence
            .split(',')
            .map(|s| s.trim().parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
            .unwrap_or_default();

        let is_originally_valid = rules.iter().all(|rule| {
            if let (Some(first_pos), Some(second_pos)) = (
                nums.iter().position(|&x| x == rule.first),
                nums.iter().position(|&x| x == rule.second),
            ) {
                first_pos < second_pos
            } else {
                true
            }
        });

        if is_originally_valid {
            println!("Sequence {}: Already valid", i + 1);
            valid_count += 1;
            original_sum += nums[nums.len() / 2];
        } else {
            sequences_to_fix.push((i + 1, sequence));
        }
    }

    // Second pass: Fix invalid sequences (aka part 2)
    let mut fixed_sum = 0;
    for (i, sequence) in sequences_to_fix {
        let result = validate_sequence(sequence, &rules);
        if let Some(fixed_sequence) = result.fixed_sequence {
            println!("Sequence {} Fixed to {:?}", i, fixed_sequence);
            valid_count += 1;
            fixed_sum += fixed_sequence[fixed_sequence.len() / 2];
        }
    }
    println!(
        "\nTotal valid sequences: {}/{}",
        valid_count,
        sequences.len()
    );
    println!(
        "Part 1: Sum of original valid middle numbers: {}",
        original_sum
    );
    println!(
        "Part 2: Sum of fixed sequence middle numbers: {}",
        fixed_sum
    );
}
