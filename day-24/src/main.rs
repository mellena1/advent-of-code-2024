use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    println!("{:?}", device.fix_instrs_for_adders(&mut vec![]));

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
    fn run_all_instructions(&mut self) -> Option<()> {
        let mut instrs_to_do = VecDeque::from_iter(self.instructions.iter());

        let mut iters_without_an_instr = 0;
        while let Some(instr) = instrs_to_do.pop_front() {
            if !self.can_do_instruction(&instr) {
                instrs_to_do.push_back(instr);
                iters_without_an_instr += 1;

                if iters_without_an_instr == instrs_to_do.len() {
                    return None;
                }

                continue;
            }

            iters_without_an_instr = 0;

            let (c, c_val) = instr.run(&self.gate_values);
            self.gate_values.insert(c, c_val);
        }

        Some(())
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

    fn swap_instructions(&mut self, swap: &(usize, usize)) {
        let a = self.instructions[swap.0].c.clone();
        let b = self.instructions[swap.1].c.clone();
        self.instructions[swap.0].c = b;
        self.instructions[swap.1].c = a;
    }

    fn fix_instrs_for_adders(
        &mut self,
        swaps: &mut Vec<(usize, usize)>,
    ) -> Option<Vec<(usize, usize)>> {
        let mut wrong_gates = self.find_wrong_z_gates_for_adder();
        wrong_gates.sort_by(|a, b| a.cmp(b));

        if swaps.len() > 4 {
            return None;
        }

        if swaps.len() == 4 && wrong_gates.len() == 0 {
            return Some(swaps.to_vec());
        }

        println!("{} {:?}", wrong_gates[0], swaps);

        let first_broken = wrong_gates[0].clone();
        let relevant_instrs = if let Ok(v) = self.find_instrs_used_for_z(&first_broken) {
            v
        } else {
            return None;
        };
        println!("{:?}", relevant_instrs);

        for (i, _) in relevant_instrs {
            for j in 0..self.instructions.len() {
                let swap = (i, j);

                self.swap_instructions(&swap);

                if self.z_gate_is_correct_for_adder(
                    &first_broken,
                    &self.get_base_inputs_for_gate(&first_broken),
                ) {
                    swaps.push(swap);
                    if let Some(ans) = self.fix_instrs_for_adders(swaps) {
                        return Some(ans);
                    } else {
                        swaps.pop();
                    }
                }

                self.swap_instructions(&swap);
            }
        }

        None
    }

    fn get_z_gate_base_inputs(&self) -> HashMap<String, Vec<Instruction>> {
        let mut map = HashMap::new();

        self.instructions
            .iter()
            .filter_map(|instr| {
                if instr.c.starts_with("z") {
                    Some(instr.c.to_string())
                } else {
                    None
                }
            })
            .for_each(|z_gate| {
                map.insert(z_gate.to_string(), self.get_base_inputs_for_gate(&z_gate));
            });

        map
    }

    fn get_base_inputs_for_gate(&self, gate: &str) -> Vec<Instruction> {
        let mut queue = vec![gate.to_string()];
        let mut instrs = vec![];
        let mut visited = HashSet::new();

        while let Some(v) = queue.pop() {
            visited.insert(v.to_string());
            if let Some(instr) = self.instructions.iter().find(|instr| instr.c == v) {
                if (instr.a.starts_with("x") || instr.a.starts_with("y"))
                    && (instr.b.starts_with("x") || instr.b.starts_with("y"))
                {
                    instrs.push(instr.clone());
                } else {
                    if !visited.contains(&instr.a) {
                        queue.push(instr.a.to_string());
                    }
                    if !visited.contains(&instr.b) {
                        queue.push(instr.b.to_string());
                    }
                }
            }
        }

        instrs
    }

    fn find_wrong_z_gates_for_adder(&self) -> Vec<String> {
        let z_gates_to_input_instrs = self.get_z_gate_base_inputs();

        z_gates_to_input_instrs
            .iter()
            .filter_map(|(z, instrs)| {
                if !self.z_gate_is_correct_for_adder(z, &instrs) {
                    Some(z.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn z_gate_is_correct_for_adder(&self, z: &str, instrs: &[Instruction]) -> bool {
        let z_bit: usize = z[1..].parse().unwrap();

        if instrs.len() != z_bit + 1 {
            return false;
        }

        let mut sorted_instrs = Vec::from(instrs);
        sorted_instrs.sort_by(|a, b| (&a.a[1..]).cmp(&b.a[1..]));

        sorted_instrs.iter().enumerate().all(|(i, instr)| {
            if i == sorted_instrs.len() - 1 {
                instr.gate == Gate::XOR && instr.a[1..].parse::<usize>().unwrap() == i
            } else {
                instr.gate == Gate::AND && instr.a[1..].parse::<usize>().unwrap() == i
            }
        })
    }

    fn find_instrs_used_for_z(&self, z: &str) -> Result<Vec<(usize, Instruction)>, ()> {
        let mut queue = vec![z.to_string()];
        let mut instrs = vec![];
        let mut visited = HashSet::new();

        while let Some(v) = queue.pop() {
            visited.insert(v.to_string());

            if let Some((i, instr)) = self
                .instructions
                .iter()
                .enumerate()
                .find(|(_, instr)| instr.c == v)
            {
                instrs.push((i, instr.clone()));
                if !visited.contains(&instr.a) {
                    queue.push(instr.a.to_string());
                }
                if !visited.contains(&instr.b) {
                    queue.push(instr.b.to_string());
                }
            }
        }

        Ok(instrs)
    }

    fn graphviz(&self) -> String {
        let mut s = "digraph {\n".to_string();

        self.instructions.iter().for_each(|instr| {
            s += format!("{} -> {}\n{} -> {}\n", instr.a, instr.c, instr.b, instr.c).as_str();
        });

        s + "}\n"
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
}
