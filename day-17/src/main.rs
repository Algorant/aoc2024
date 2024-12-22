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
        println!(
            "Executing: {:?}, operand: {}, operand_type: {:?}",
            opcode, operand, operand_type
        );
        println!(
            "Before - A: {}, B: {}, C: {}",
            self.register_a, self.register_b, self.register_c
        );
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
        println!(
            "After - A: {}, B: {}, C: {}",
            self.register_a, self.register_b, self.register_c
        );
        println!("Instruction pointer: {}\n", self.instruction_pointer);
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

fn main() {
    // Test input
    let initial_a = 729;
    let initial_b = 0;
    let initial_c = 0;
    let program = "0,1,5,4,3,0";

    let output = run_program(initial_a, initial_b, initial_c, program);
    println!("Program output: {}", output);
}
