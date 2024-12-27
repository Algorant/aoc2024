use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Button {
    Num(u8),
    A,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DPadButton {
    Up,
    Down,
    Left,
    Right,
    A,
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

struct DirectionalPad {
    current_pos: Position,
    layout: [[Option<DPadButton>; 3]; 2],
}

struct Keypad {
    current_pos: Position,
    layout: [[Option<Button>; 3]; 4],
}

impl DirectionalPad {
    fn new() -> Self {
        let mut layout = [[None; 3]; 2];

        // Fill out Dpad layout
        layout[0] = [None, Some(DPadButton::Up), Some(DPadButton::A)];
        layout[1] = [
            Some(DPadButton::Left),
            Some(DPadButton::Down),
            Some(DPadButton::Right),
        ];

        DirectionalPad {
            current_pos: Position { x: 2, y: 0 }, // Start at 'A' position
            layout,
        }
    }

    fn move_direction(&mut self, direction: &str) -> bool {
        let new_pos = match direction {
            "up" => Position {
                x: self.current_pos.x,
                y: self.current_pos.y - 1,
            },
            "down" => Position {
                x: self.current_pos.x,
                y: self.current_pos.y + 1,
            },
            "left" => Position {
                x: self.current_pos.x - 1,
                y: self.current_pos.y,
            },
            "right" => Position {
                x: self.current_pos.x + 1,
                y: self.current_pos.y,
            },
            _ => return false,
        };

        if self.is_valid_position(&new_pos) {
            self.current_pos = new_pos;
            true
        } else {
            false
        }
    }

    fn is_valid_position(&self, pos: &Position) -> bool {
        if pos.y < 0 || pos.y >= 2 || pos.x < 0 || pos.x >= 3 {
            return false;
        }

        self.layout[pos.y as usize][pos.x as usize].is_some()
    }

    fn get_current_button(&self) -> Option<DPadButton> {
        self.layout[self.current_pos.y as usize][self.current_pos.x as usize]
    }
}

impl Keypad {
    fn new() -> Self {
        let mut layout = [[None; 3]; 4];

        layout[0] = [
            Some(Button::Num(7)),
            Some(Button::Num(8)),
            Some(Button::Num(9)),
        ];
        layout[1] = [
            Some(Button::Num(4)),
            Some(Button::Num(5)),
            Some(Button::Num(6)),
        ];
        layout[2] = [
            Some(Button::Num(1)),
            Some(Button::Num(2)),
            Some(Button::Num(3)),
        ];
        layout[3] = [None, Some(Button::Num(0)), Some(Button::A)];

        Keypad {
            current_pos: Position { x: 2, y: 3 }, // Start at 'A' position
            layout,
        }
    }

    fn move_direction(&mut self, direction: &str) -> bool {
        let new_pos = match direction {
            "up" => Position {
                x: self.current_pos.x,
                y: self.current_pos.y - 1,
            },
            "down" => Position {
                x: self.current_pos.x,
                y: self.current_pos.y + 1,
            },
            "left" => Position {
                x: self.current_pos.x - 1,
                y: self.current_pos.y,
            },
            "right" => Position {
                x: self.current_pos.x + 1,
                y: self.current_pos.y,
            },
            _ => return false,
        };

        if self.is_valid_position(&new_pos) {
            self.current_pos = new_pos;
            true
        } else {
            false
        }
    }

    fn is_valid_position(&self, pos: &Position) -> bool {
        if pos.y < 0 || pos.y >= 4 || pos.x < 0 || pos.x >= 3 {
            return false;
        }
        self.layout[pos.y as usize][pos.x as usize].is_some()
    }

    fn get_current_button(&self) -> Option<Button> {
        self.layout[self.current_pos.y as usize][self.current_pos.x as usize]
    }
}

struct ControlSystem {
    dpad1: DirectionalPad,
    dpad2: DirectionalPad,
    dpad3: DirectionalPad,
    keypad: Keypad,
}

#[derive(Debug, Clone)]
struct SystemState {
    dpad3: Position,
    dpad2: Position,
    dpad1: Position,
    keypad: Position,
    sequence: Vec<String>,     // Stores sequence of moves so far
    code_entered: Vec<Button>, // Stores buttons successfully entered
}

impl ControlSystem {
    fn new() -> Self {
        ControlSystem {
            dpad1: DirectionalPad::new(),
            dpad2: DirectionalPad::new(),
            dpad3: DirectionalPad::new(),
            keypad: Keypad::new(),
        }
    }

    fn process_input(&mut self, input: &str) -> Option<Button> {
        // Special handling for activate command
        if input == "activate" {
            // Check if we're on 'A' for the current active pad
            match self.dpad3.get_current_button() {
                Some(DPadButton::A) => {
                    // dpad3 is on 'A', check dpad2
                    match self.dpad2.get_current_button() {
                        Some(DPadButton::A) => {
                            // dpad2 is on 'A', check dpad1
                            match self.dpad1.get_current_button() {
                                Some(DPadButton::A) => {
                                    // All pads are on 'A', return keypad button
                                    return self.keypad.get_current_button();
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        } else {
            // Move dpad3
            if !self.dpad3.move_direction(input) {
                return None;
            }
        }

        // Get action from dpad3
        let dpad3_action = match self.dpad3.get_current_button() {
            Some(DPadButton::A) => "activate",
            Some(DPadButton::Up) => "up",
            Some(DPadButton::Down) => "down",
            Some(DPadButton::Left) => "left",
            Some(DPadButton::Right) => "right",
            None => return None,
        };

        // If not activate action, move dpad2
        if dpad3_action != "activate" {
            if !self.dpad2.move_direction(dpad3_action) {
                return None;
            }
            return None;
        }

        // Get action from dpad2
        let dpad2_action = match self.dpad2.get_current_button() {
            Some(DPadButton::A) => "activate",
            Some(DPadButton::Up) => "up",
            Some(DPadButton::Down) => "down",
            Some(DPadButton::Left) => "left",
            Some(DPadButton::Right) => "right",
            None => return None,
        };

        // If not activate action, move dpad1
        if dpad2_action != "activate" {
            if !self.dpad1.move_direction(dpad2_action) {
                return None;
            }
            return None;
        }

        // Get action from dpad1
        let dpad1_action = match self.dpad1.get_current_button() {
            Some(DPadButton::A) => "activate",
            Some(DPadButton::Up) => "up",
            Some(DPadButton::Down) => "down",
            Some(DPadButton::Left) => "left",
            Some(DPadButton::Right) => "right",
            None => return None,
        };

        // If not activate action, move keypad
        if dpad1_action != "activate" {
            if !self.keypad.move_direction(dpad1_action) {
                return None;
            }
            return None;
        };
        // If we get here, all three dpads were "activated", so return the current keypad button

        self.keypad.get_current_button()
    }

    fn find_sequence_for_code(&self, target_code: Vec<Button>) -> Option<Vec<String>> {
        println!("Starting search for code: {:?}", target_code);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Initial state
        let initial_state = SystemState {
            dpad3: self.dpad3.current_pos,
            dpad2: self.dpad2.current_pos,
            dpad1: self.dpad1.current_pos,
            keypad: self.keypad.current_pos,
            sequence: Vec::new(),
            code_entered: Vec::new(),
        };

        queue.push_back(initial_state);

        while let Some(current_state) = queue.pop_front() {
            // Check if we've completed the code
            if current_state.code_entered == target_code {
                return Some(current_state.sequence);
            }

            // Generate state key for visited check
            let state_key = (
                current_state.dpad3,
                current_state.dpad2,
                current_state.dpad1,
                current_state.keypad,
                current_state.code_entered.len(),
            );
            if visited.contains(&state_key) {
                continue;
            }

            visited.insert(state_key);

            // Try each possible move
            for direction in &["up", "down", "left", "right", "activate"] {
                let mut new_state = current_state.clone();
                new_state.sequence.push(direction.to_string());

                // Create temporary control system to simulate move
                let mut temp_system = ControlSystem::new();
                temp_system.dpad3.current_pos = new_state.dpad3;
                temp_system.dpad2.current_pos = new_state.dpad2;
                temp_system.dpad1.current_pos = new_state.dpad1;
                temp_system.keypad.current_pos = new_state.keypad;

                // Try to process input
                if let Some(button) = temp_system.process_input(direction) {
                    println!("Found button press: {:?} after move: {}", button, direction);
                    let mut new_code = new_state.code_entered.clone();
                    new_code.push(button);

                    // Check if this matches what we want
                    if new_code.len() <= target_code.len()
                        && new_code[new_code.len() - 1] == target_code[new_code.len() - 1]
                    {
                        new_state.code_entered = new_code;
                        new_state.dpad3 = temp_system.dpad3.current_pos;
                        new_state.dpad2 = temp_system.dpad2.current_pos;
                        new_state.dpad1 = temp_system.dpad1.current_pos;
                        new_state.keypad = temp_system.keypad.current_pos;
                        queue.push_back(new_state);
                    }
                } else {
                    // Move was valid but didn't result in a button press
                    new_state.dpad3 = temp_system.dpad3.current_pos;
                    new_state.dpad2 = temp_system.dpad2.current_pos;
                    new_state.dpad1 = temp_system.dpad1.current_pos;
                    new_state.keypad = temp_system.keypad.current_pos;
                    queue.push_back(new_state);
                }
            }
        }
        None // No solution found
    }
}

fn main() {
    let mut control_system = ControlSystem::new();

    // Test a single move sequence first
    println!("Testing single button press:");
    let result = control_system.process_input("activate");
    println!("Result of single activate: {:?}", result);

    // Now try to find the sequence for the code
    let control_system = ControlSystem::new();
    let target_code = vec![Button::Num(1), Button::Num(2), Button::Num(3), Button::A];

    match control_system.find_sequence_for_code(target_code) {
        Some(sequence) => {
            println!("Found sequence:");
            for (i, step) in sequence.iter().enumerate() {
                println!("Step {}: {}", i + 1, step);
            }
        }
        None => println!("No solution found!"),
    }
}
