#[derive(Debug)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instruction_pointer: i64,
    output: Vec<i64>,
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADV = 0, // Divide A by 2^operand -> A
    BXL = 1, // B XOR literal -> B
    BST = 2, // Poerand Mod 8 -> B
    JNZ = 3, // Jump if A !=0
    BXC = 4, // B XOR C -> B
    OUT = 5, // Output opernad mod 8
    BDV = 6, // Divide A by 2^operand -> B
    CDV = 7, // Divide A by 2^operand -> C
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0 => Opcode::ADV,
            1 => Opcode::BXL,
            2 => Opcode::BST,
            3 => Opcode::JNZ,
            4 => Opcode::BXC,
            5 => Opcode::OUT,
            6 => Opcode::BDV,
            7 => Opcode::CDV,
            _ => panic!("Invalid opcode"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum OperandType {
    Literal,
    Combo,
}

impl Computer {
    fn new() -> Self {
        Computer {
            register_a: 0,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            output: Vec::new(),
        }
    }

    fn execute_instruction(&mut self, opcode: Opcode, operand: u8, operand_type: OperandType) {
        // Debug output removed for searching
        match opcode {
            Opcode::ADV => {
                let power = self.resolve_operand(operand, operand_type);
                self.register_a = self.register_a / (1 << power);
            }
            Opcode::BXL => {
                let value = self.resolve_operand(operand, OperandType::Literal);
                self.register_b ^= value;
            }
            Opcode::BST => {
                let value = self.resolve_operand(operand, operand_type) % 8;
                self.register_b = value;
            }
            Opcode::JNZ => {
                if self.register_a != 0 {
                    self.instruction_pointer =
                        self.resolve_operand(operand, OperandType::Literal) as i64;
                    return; // Skip the normal instructing pointer increment
                }
            }
            Opcode::BXC => {
                self.register_b ^= self.register_c; // Operand is ignored
            }
            Opcode::OUT => {
                let value = self.resolve_operand(operand, operand_type) % 8;
                self.output.push(value);
            }
            Opcode::BDV => {
                let power = self.resolve_operand(operand, operand_type);
                self.register_b = self.register_a / (1 << power);
            }
            Opcode::CDV => {
                let power = self.resolve_operand(operand, operand_type);
                self.register_c = self.register_a / (1 << power);
            }
        }
        self.instruction_pointer += 2; // Normal instruction pointer increment
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn get_register_value(&self, register: u8) -> Option<i64> {
        match register {
            4 => Some(self.register_a),
            5 => Some(self.register_b),
            6 => Some(self.register_c),
            _ => None,
        }
    }

    fn resolve_operand(&self, operand: u8, operand_type: OperandType) -> i64 {
        match operand_type {
            OperandType::Literal => operand as i64,
            OperandType::Combo => match operand {
                0..=3 => operand as i64,
                4..=6 => self.get_register_value(operand).unwrap(),
                7 => panic!("Invalid combo operand 7"),
                _ => panic!("Invalid combo operand > 7"),
            },
        }
    }
}

fn run_program(initial_a: i64, initial_b: i64, initial_c: i64, program: &str) -> String {
    let mut computer = Computer::new();

    // Set initial register values
    computer.register_a = initial_a;
    computer.register_b = initial_b;
    computer.register_c = initial_c;

    // Parse program string into instructions
    let instructions: Vec<u8> = program
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // Execute program
    while computer.instruction_pointer < instructions.len() as i64 {
        let opcode = Opcode::from(instructions[computer.instruction_pointer as usize]);
        let operand = instructions[(computer.instruction_pointer + 1) as usize];

        // Determine operand type based on opcode
        let operand_type = match opcode {
            Opcode::BXL | Opcode::JNZ => OperandType::Literal,
            _ => OperandType::Combo,
        };

        computer.execute_instruction(opcode, operand, operand_type);
    }

    computer.get_output()
}

fn find_self_replicating_a(program: &str) -> Option<i64> {
    // Parse target program into numbers
    let target_numbers: Vec<i64> = program
        .split(',')
        .filter_map(|s| s.parse().ok())
        .rev() // Reverse because we build from right to left
        .collect();

    println!("Target (reversed): {:?}", target_numbers);

    // Start with just testing rightomst digit (first in our reversed list)
    let mut candidates = vec![(0i64, Vec::new())]; // (value, matching_outputs)

    // For each position (right to left)
    for (pos, &target) in target_numbers.iter().enumerate() {
        println!("\nTesting position {} (target={})", pos, target);
        let mut new_candidates = Vec::new();

        // For each candidate from previous position
        for (prev_value, prev_outputs) in candidates {
            // Try all possible 3-bit values (0-7)
            for i in 0..8 {
                let test_value = (prev_value << 3) | i;
                let output = run_program(test_value, 0, 0, program);
                let output_nums: Vec<i64> =
                    output.split(',').filter_map(|s| s.parse().ok()).collect();

                // Check if this value produces the correct sequence so far
                if output_nums.len() > pos
                    && output_nums[..=pos]
                        .iter()
                        .rev()
                        .copied()
                        .collect::<Vec<_>>()
                        == target_numbers[..=pos]
                {
                    let mut new_outputs = prev_outputs.clone();
                    new_outputs.push(i);
                    new_candidates.push((test_value, new_outputs));
                    println!("Found candidate: {} -> {:?}", test_value, output);
                }
            }
        }

        if new_candidates.is_empty() {
            println!("No candidates found for position {}", pos);
            return None;
        }

        candidates = new_candidates;
    }

    // Return the smallest value that generates the complete sequence
    candidates.into_iter().map(|(value, _)| value).min()
}

fn main() {
    let program = "2,4,1,5,7,5,1,6,4,1,5,5,0,3,3,0";

    // Part 1
    let part1_a = 60589763;
    println!("Part 1:");
    let output = run_program(part1_a, 0, 0, program);
    println!("Program output with A={}: {}\n", part1_a, output);

    // Part 2
    println!("Part 2:");
    println!("Searching for self-replicating value of A...");
    match find_self_replicating_a(program) {
        Some(a) => println!("Found self-replicating value: A={}", a),
        None => println!("No self-replicating value found in search range"),
    }
}
