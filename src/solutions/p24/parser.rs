use super::machine::*;
use std::fs::File;
use std::io::{self, BufRead};

enum InputState {
    Input,
    Gates,
}

pub fn parse() -> Machine {
    let file = File::open("src/solutions/p24/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut state = InputState::Input;
    let mut machine = Machine::new();
    for line in reader.lines() {
        let line = line.unwrap();

        match state {
            InputState::Input => {
                if line.is_empty() {
                    state = InputState::Gates;
                    continue;
                }

                let data: Vec<_> = line.split(": ").collect();
                let name = data[0].to_string();
                let value = data[1].parse::<u64>().unwrap();
                let wire = Wire::new(name, Some(value));
                machine.add_input(wire);
            }
            InputState::Gates => {
                let data: Vec<_> = line.split(" ").collect();
                let input1 = data[0].to_string();
                let operation = Operation::from_string(data[1]);
                let input2 = data[2].to_string();
                let output = data[4].to_string();
                let gate = Gate::new(input1, input2, output, operation);
                machine.add_gate(gate);
            }
        }
    }

    machine
}
