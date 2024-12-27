use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wire {
    pub name: String,
    pub value: Option<u64>,
}

impl Wire {
    pub fn new(name: String, value: Option<u64>) -> Self {
        Self { name, value: value }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    AND,
    OR,
    XOR,
}

impl Operation {
    fn apply(&self, input1: u64, input2: u64) -> u64 {
        match self {
            Operation::AND => input1 & input2,
            Operation::OR => input1 | input2,
            Operation::XOR => input1 ^ input2,
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Gate {
    pub input1: Wire,
    pub input2: Wire,
    pub output: Wire,
    pub operation: Operation,
}

impl Gate {
    pub fn new(input1: String, input2: String, output: String, operation: Operation) -> Self {
        Self {
            input1: Wire::new(input1, None),
            input2: Wire::new(input2, None),
            output: Wire::new(output, None),
            operation,
        }
    }

    fn update(&mut self) -> bool {
        if self.input1.value.is_none() || self.input2.value.is_none() {
            return false;
        }
        let value = self
            .operation
            .apply(self.input1.value.unwrap(), self.input2.value.unwrap());
        self.output.value = Some(value);
        true
    }

    /// Returns true if an update was actually made
    pub fn update_wire(&mut self, wire: &Wire) -> bool {
        if self.input1.value.is_some() && self.input2.value.is_some() {
            return false;
        }
        if wire.name == self.input1.name {
            self.input1.value = Some(wire.value.unwrap());
        }
        if wire.name == self.input2.name {
            self.input2.value = Some(wire.value.unwrap());
        }
        self.update()
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub wires: Vec<Wire>,
    pub gates: Vec<Gate>,
    pub input_size: usize,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            wires: Vec::new(),
            gates: Vec::new(),
            input_size: 0,
        }
    }
    pub fn add_input(&mut self, wire: Wire) {
        self.wires.push(wire);
        self.input_size = self.wires.len() / 2;
    }
    pub fn add_gate(&mut self, gate: Gate) {
        self.gates.push(gate);
    }

    pub fn swap_outputs(&mut self, i: usize, j: usize) {
        let temp = self.gates[i].output.clone();
        self.gates[i].output = self.gates[j].output.clone();
        self.gates[j].output = temp;
    }

    fn _set_input(&mut self, name_prefix: char, value: u64) {
        let mut mask = 1;
        for i in 0..self.input_size {
            let bit = (value & mask) >> i;
            let name = format!("{}{:02}", name_prefix, i);
            let wire = Wire::new(name, Some(bit));
            self.add_input(wire);
            mask <<= 1;
        }
    }

    fn set_input(&mut self, value1: u64, value2: u64) {
        self.wires.clear();
        self._set_input('x', value1);
        self._set_input('y', value2);
    }

    fn reset_gates(&mut self) {
        self.gates.iter_mut().for_each(|gate| {
            gate.input1.value = None;
            gate.input2.value = None;
            gate.output.value = None;
        });
    }

    /// Sets the input of the machine to the given values and clears all gates.
    pub fn reset(&mut self, value1: u64, value2: u64) {
        self.set_input(value1, value2);
        self.reset_gates();
    }

    pub fn run(&mut self) -> u64 {
        let mut new_wires = vec![];

        // Run a simulation until there are no new wires to update.
        let mut queue = VecDeque::new();
        queue.extend(self.wires.clone());

        while let Some(wire) = queue.pop_front() {
            for gate in self.gates.iter_mut() {
                if gate.update_wire(&wire) {
                    queue.push_back(gate.output.clone());
                    new_wires.push(gate.output.clone());
                }
            }
        }

        // Extract the bits from wires starting with 'z'
        let output = new_wires
            .iter()
            .filter(|wire| wire.name.starts_with("z"))
            .sorted()
            .rev() // sort bits
            .map(|wire| wire.value.unwrap())
            .fold(0, |acc, bit| (acc << 1) | bit);

        output
    }
}
