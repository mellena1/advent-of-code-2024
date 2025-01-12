use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

fn main() {
    let mut device = read_input("input.txt").expect("failed to read input");
    let orig_gate_values = device.gate_values.clone();

    println!("Part 1: {}", part1(&mut device));

    device.gate_values = orig_gate_values;
    println!("Part 2: {}", part2(&mut device));
}

fn part1(device: &mut Device) -> usize {
    device.run_all_instructions();
    device.get_num_from_z_gates()
}

fn part2(device: &mut Device) -> String {
    let orig_instrs = device.instructions.clone();

    let mut x_gates: Vec<_> = device
        .gate_values
        .iter()
        .filter(|(k, _)| k.starts_with("x"))
        .collect();
    let x_num = get_num_from_list_of_gates(&mut x_gates);

    let mut y_gates: Vec<_> = device
        .gate_values
        .iter()
        .filter(|(k, _)| k.starts_with("y"))
        .collect();
    let y_num = get_num_from_list_of_gates(&mut y_gates);

    let needed_z = x_num + y_num;

    "".to_string()
}

fn read_input(path: &str) -> Result<Device, std::io::Error> {
    let input = read_to_string(&path)?;
    Ok(Device::from(input.as_str()))
}

struct Device {
    gate_values: HashMap<String, bool>,
    instructions: Vec<Instruction>,
}

impl Device {
    fn run_all_instructions(&mut self) {
        let mut instrs_to_do = VecDeque::from_iter(self.instructions.iter());

        while let Some(instr) = instrs_to_do.pop_front() {
            if !self.can_do_instruction(&instr) {
                instrs_to_do.push_back(instr);
                continue;
            }

            let (c, c_val) = instr.run(&self.gate_values);
            self.gate_values.insert(c, c_val);
        }
    }

    fn get_num_from_z_gates(&self) -> usize {
        let mut z_gates: Vec<_> = self
            .gate_values
            .iter()
            .filter(|(k, _)| k.starts_with("z"))
            .collect();

        get_num_from_list_of_gates(&mut z_gates)
    }

    fn can_do_instruction(&self, instr: &Instruction) -> bool {
        self.gate_values.contains_key(&instr.a) && self.gate_values.contains_key(&instr.b)
    }
}

impl From<&str> for Device {
    fn from(value: &str) -> Self {
        let (gates, instrs) = value.split_once("\n\n").unwrap();

        let gate_values = gates
            .lines()
            .map(|g| {
                let (k, v) = g.split_once(": ").expect("bad gate str");

                (k.to_string(), v == "1")
            })
            .collect();

        let instructions = instrs.lines().map(|line| Instruction::from(line)).collect();

        Self {
            gate_values,
            instructions,
        }
    }
}

#[derive(Clone)]
struct Instruction {
    a: String,
    gate: Gate,
    b: String,

    c: String,
}

impl Instruction {
    fn run(&self, gate_values: &HashMap<String, bool>) -> (String, bool) {
        let a_val = gate_values[&self.a];
        let b_val = gate_values[&self.b];
        let c_val = match self.gate {
            Gate::AND => a_val && b_val,
            Gate::OR => a_val || b_val,
            Gate::XOR => a_val ^ b_val,
        };
        (self.c.to_string(), c_val)
    }
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let without_arrow = value.replace("->", "");
        let split: Vec<_> = without_arrow.split_whitespace().collect();

        Self {
            a: split[0].to_string(),
            gate: Gate::from(split[1]),
            b: split[2].to_string(),
            c: split[3].to_string(),
        }
    }
}

#[derive(Clone)]
enum Gate {
    AND,
    OR,
    XOR,
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::AND,
            "OR" => Self::OR,
            "XOR" => Self::XOR,
            _ => panic!("unknown gate"),
        }
    }
}

fn get_num_from_list_of_gates(gates: &mut Vec<(&String, &bool)>) -> usize {
    gates.sort_by(|a, b| a.0.cmp(&b.0));

    let mut result = 0;

    gates.iter().enumerate().for_each(|(i, (_, v))| {
        if **v {
            result |= 1 << i;
        }
    });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut device = read_input("example.txt").expect("failed to read input");
        assert_eq!(part1(&mut device), 2024);
    }

    #[test]
    fn part2_works() {
        let mut device = read_input("example2.txt").expect("failed to read input");
        assert_eq!(part2(&mut device), "z00,z01,z02,z05".to_string());
    }
}
