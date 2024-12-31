use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    fn is_valid_chain_position(&self) -> bool {
        // Check if any pad is over an invalid position on the keypad
        let keypad_positions = [
            self.dpad1.current_pos,
            self.dpad2.current_pos,
            self.dpad3.current_pos,
        ];

        for pos in keypad_positions.iter() {
            // Convert dpad position to equivalent keypad position
            let keypad_x = pos.x;
            let keypad_y = pos.y;

            // Check if this position would be invalid on the keypad
            if keypad_y < 0 || keypad_y >= 4 || keypad_x < 0 || keypad_x >= 3 {
                return false;
            }

            // Check if this position corresponds to None on the keypad
            if self.keypad.layout[keypad_y as usize][keypad_x as usize].is_none() {
                return false;
            }
        }
        true
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
            // Check if this movement created an invalid chain position
            if !self.is_valid_chain_position() {
                // Undo the movement
                self.dpad3.current_pos = Position { x: 2, y: 0 };
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
            // Check if this movement created an invalid chain position
            if !self.is_valid_chain_position() {
                // Undo the movement
                self.dpad2.current_pos = Position { x: 2, y: 0 };
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
            // Check if this movement created an invalid chain position
            if !self.is_valid_chain_position() {
                // Undo the movement
                self.dpad1.current_pos = Position { x: 2, y: 0 };
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
            // Check if this movement created an invalid chain position
            if !self.is_valid_chain_position() {
                // Undo the movement
                self.keypad.current_pos = Position { x: 2, y: 3 };
                return None;
            }
            return None;
        };
        // If we get here, all three dpads were "activated", so return the current keypad button

        self.keypad.get_current_button()
    }

    fn find_sequence_for_code(&self, target_code: Vec<Button>) -> Option<Vec<String>> {
        use std::cmp::Ordering;
        use std::collections::BinaryHeap;

        #[derive(Eq, Clone)]
        struct State {
            cost: usize,
            pos: (Position, Position, Position, Position),
            sequence: Vec<String>,
            code_entered: Vec<Button>,
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                other.cost.cmp(&self.cost)
            }
        }

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for State {
            fn eq(&self, other: &Self) -> bool {
                self.cost == other.cost
            }
        }

        let mut distances: HashMap<(Position, Position, Position, Position, usize), usize> =
            HashMap::new();
        let mut heap = BinaryHeap::new();

        // Initial state
        let initial_pos = (
            self.dpad3.current_pos,
            self.dpad2.current_pos,
            self.dpad1.current_pos,
            self.keypad.current_pos,
        );

        heap.push(State {
            cost: 0,
            pos: initial_pos,
            sequence: Vec::new(),
            code_entered: Vec::new(),
        });

        distances.insert(
            (
                initial_pos.0,
                initial_pos.1,
                initial_pos.2,
                initial_pos.3,
                0,
            ),
            0,
        );

        while let Some(State {
            cost,
            pos,
            sequence,
            code_entered,
        }) = heap.pop()
        {
            // Check if we've completed the code
            if code_entered == target_code {
                return Some(sequence);
            }

            let state_key = (pos.0, pos.1, pos.2, pos.3, code_entered.len());

            // If we've found a longer path to this state, skip it
            if let Some(&d) = distances.get(&state_key) {
                if cost > d {
                    continue;
                }
            }

            // Try each possible move
            for direction in &["up", "down", "left", "right", "activate"] {
                let mut new_sequence = sequence.clone();
                new_sequence.push(direction.to_string());

                // Create temporary control system to simulate move
                let mut temp_system = ControlSystem::new();
                temp_system.dpad3.current_pos = pos.0;
                temp_system.dpad2.current_pos = pos.1;
                temp_system.dpad1.current_pos = pos.2;
                temp_system.keypad.current_pos = pos.3;

                // Try to process input
                if let Some(button) = temp_system.process_input(direction) {
                    //println!("Found button press: {:?} after move: {}", button, direction);
                    let mut new_code = code_entered.clone();
                    new_code.push(button);

                    if new_code.len() <= target_code.len()
                        && new_code[new_code.len() - 1] == target_code[new_code.len() - 1]
                    {
                        let new_pos = (
                            temp_system.dpad3.current_pos,
                            temp_system.dpad2.current_pos,
                            temp_system.dpad1.current_pos,
                            temp_system.keypad.current_pos,
                        );

                        let new_cost = cost + 1;
                        let new_key = (new_pos.0, new_pos.1, new_pos.2, new_pos.3, new_code.len());

                        if !distances.contains_key(&new_key) || distances[&new_key] > new_cost {
                            distances.insert(new_key, new_cost);
                            heap.push(State {
                                cost: new_cost,
                                pos: new_pos,
                                sequence: new_sequence,
                                code_entered: new_code,
                            });
                        }
                    }
                } else {
                    // Move was valid but didn't result in a button press
                    let new_pos = (
                        temp_system.dpad3.current_pos,
                        temp_system.dpad2.current_pos,
                        temp_system.dpad1.current_pos,
                        temp_system.keypad.current_pos,
                    );

                    let new_cost = cost + 1;
                    let new_key = (
                        new_pos.0,
                        new_pos.1,
                        new_pos.2,
                        new_pos.3,
                        code_entered.len(),
                    );

                    if !distances.contains_key(&new_key) || distances[&new_key] > new_cost {
                        distances.insert(new_key, new_cost);
                        heap.push(State {
                            cost: new_cost,
                            pos: new_pos,
                            sequence: new_sequence,
                            code_entered: code_entered.clone(),
                        });
                    }
                }
            }
        }
        None // No solution found
    }
}

fn main() {
    let codes = vec![
        (
            "789A",
            vec![Button::Num(7), Button::Num(8), Button::Num(9), Button::A],
        ),
        (
            "540A",
            vec![Button::Num(5), Button::Num(4), Button::Num(0), Button::A],
        ),
        (
            "285A",
            vec![Button::Num(2), Button::Num(8), Button::Num(5), Button::A],
        ),
        (
            "140A",
            vec![Button::Num(1), Button::Num(4), Button::Num(0), Button::A],
        ),
        (
            "189A",
            vec![Button::Num(1), Button::Num(8), Button::Num(9), Button::A],
        ),
    ];

    let mut total_sum = 0;

    for (code_str, target_code) in codes {
        let control_system = ControlSystem::new();
        match control_system.find_sequence_for_code(target_code) {
            Some(sequence) => {
                // Extract numeric part from code_str by removing 'A'
                let numeric_part: u32 = code_str.trim_end_matches('A').parse().unwrap();
                let complexity = sequence.len() as u32 * numeric_part;
                println!(
                    "Code {}: {} moves, complexity: {}",
                    code_str,
                    sequence.len(),
                    complexity
                );
                total_sum += complexity;
            }
            None => println!("No solution found for code {}", code_str),
        }
    }

    println!("Total sum of complexities: {}", total_sum);
}
