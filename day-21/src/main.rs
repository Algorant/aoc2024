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

#[derive(Debug, Clone, Copy, PartialEq)]
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
            // No need to move, just get the current button and process
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
}

fn main() {
    let mut control_system = ControlSystem::new();

    // Step 1: Move dpad3 to 'A' and activate it
    println!(
        "Current dpad3 button: {:?}",
        control_system.dpad3.get_current_button()
    );
    // dpad3 starts on 'A', so we can activate immediately
    let result = control_system.process_input("activate");
    println!("After activating dpad3: {:?}", result);

    // Step 2: Move dpad2 to 'A' and activate it
    println!(
        "Current dpad2 button: {:?}",
        control_system.dpad2.get_current_button()
    );
    // dpad2 starts on 'A', so we can activate immediately
    let result = control_system.process_input("activate");
    println!("After activating dpad2: {:?}", result);

    // Step 3: Move dpad1 to 'A' and activate it
    println!(
        "Current dpad1 button: {:?}",
        control_system.dpad1.get_current_button()
    );
    // dpad1 starts on 'A', so we can activate immediately
    let result = control_system.process_input("activate");
    println!("After activating dpad1: {:?}", result);

    // Final result should be pressing 'A' on the keypad
    println!("Final result: {:?}", result);
}
