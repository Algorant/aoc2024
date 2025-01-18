use itertools::Itertools;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn execute(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Clone, Copy)]
struct Operation<'a> {
    lhs: &'a str,
    op: Operator,
    rhs: &'a str,
}

fn parse(input: &str) -> (HashMap<&str, bool>, HashMap<&str, Operation>) {
    let (top, bottom) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    for line in top.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        wires.insert(left, right == "1");
    }
    let mut operations = HashMap::new();
    for line in bottom.lines() {
        let (left, right) = line.split_once(" -> ").unwrap();
        let (lhs, op, rhs) = left.split_whitespace().collect_tuple().unwrap();
        let op = match op {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("at the disco"),
        };
        operations.insert(right, Operation { lhs, op, rhs });
    }
    (wires, operations)
}

fn calc<'a>(
    wires: &mut HashMap<&'a str, bool>,
    ops: &HashMap<&'a str, Operation<'a>>,
    wire: &'a str,
) -> bool {
    if let Some(&on) = wires.get(wire) {
        return on;
    }
    let Operation { lhs, op, rhs } = &ops[wire];
    let lhs = calc(wires, ops, lhs);
    let rhs = calc(wires, ops, rhs);
    let res = op.execute(lhs, rhs);
    wires.insert(wire, res);
    res
}

fn part_1(input: &str) -> u64 {
    let (mut wires, ops) = parse(input);
    ops.keys()
        // get all wires that start with z and sort them
        .filter(|name| name.starts_with('z'))
        .sorted()
        // least significant bit is first, reverse
        .rev()
        // calculate the bits those wires output
        .map(|name| calc(&mut wires, &ops, name))
        // concatenate the bits (with boolean math!)
        .fold(0, |acc, bit| acc << 1 | bit as u64)
}

fn make_wire(c: char, n: i32) -> String {
    format!("{}{:02}", c, n)
}

fn is_ok_z(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::Xor {
            return false;
        }
        if num == 0 {
            let mut operands = [*lhs, *rhs];
            operands.sort();
            return operands == ["x00", "y00"];
        }
        return (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
            || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num));
    }
    false
}

fn is_ok_xor(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::Xor {
            return false;
        }
        let mut operands = [*lhs, *rhs];
        operands.sort();
        return operands == [make_wire('x', num), make_wire('y', num)];
    }
    false
}

fn is_ok_carry_bit(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if num == 1 {
            if *op != Operator::And {
                return false;
            }
            let mut operands = [*lhs, *rhs];
            operands.sort();
            return operands == ["x00", "y00"];
        }
        if *op != Operator::Or {
            return false;
        }
        return (is_ok_direct_carry(ops, lhs, num - 1) && is_ok_recarry(ops, rhs, num - 1))
            || (is_ok_direct_carry(ops, rhs, num - 1) && is_ok_recarry(ops, lhs, num - 1));
    }
    false
}

fn is_ok_direct_carry(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::And {
            return false;
        }
        let mut operands = [*lhs, *rhs];
        operands.sort();
        return operands == [make_wire('x', num), make_wire('y', num)];
    }
    false
}

fn is_ok_recarry(ops: &HashMap<&str, Operation>, wire: &str, num: i32) -> bool {
    if let Some(Operation { lhs, op, rhs }) = ops.get(wire) {
        if *op != Operator::And {
            return false;
        }
        return (is_ok_xor(ops, lhs, num) && is_ok_carry_bit(ops, rhs, num))
            || (is_ok_xor(ops, rhs, num) && is_ok_carry_bit(ops, lhs, num));
    }
    false
}

fn progress(ops: &HashMap<&str, Operation>) -> i32 {
    (0..)
        .find(|&idx| !is_ok_z(ops, &make_wire('z', idx), idx))
        .unwrap()
}

fn swap_wires<'a>(map: &mut HashMap<&'a str, Operation<'a>>, a: &'a str, b: &'a str) {
    let temp = map[a];
    map.insert(a, map[b]);
    map.insert(b, temp);
}

fn part_2(input: &str) -> String {
    let (_, mut ops) = parse(input);
    let mut swaps = Vec::new();

    let wires: Vec<&str> = ops.keys().copied().collect();
    for _ in 0..4 {
        let baseline = progress(&ops);
        for (a, b) in wires.iter().tuple_combinations() {
            swap_wires(&mut ops, a, b);
            if progress(&ops) > baseline {
                swaps.push([*a, *b]);
                break;
            }
            swap_wires(&mut ops, a, b);
        }
    }

    swaps
        .into_iter()
        .flatten()
        .sorted()
        .intersperse(",")
        .collect()
}

fn main() -> std::io::Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

