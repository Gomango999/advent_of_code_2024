use super::machine::Machine;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn read_next_line(reader: &mut io::BufReader<File>) -> String {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line
}

pub fn parse() -> Machine {
    let file = File::open("src/solutions/p17/in.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let re1 = Regex::new(r"Register .: (\d+)").unwrap();
    let re2 = Regex::new(r"Program: ([\d,]+)").unwrap();

    let line = read_next_line(&mut reader);
    let (_, [register_a]) = re1.captures(&line).unwrap().extract();
    let line = read_next_line(&mut reader);
    let (_, [register_b]) = re1.captures(&line).unwrap().extract();
    let line = read_next_line(&mut reader);
    let (_, [register_c]) = re1.captures(&line).unwrap().extract();
    let _ = read_next_line(&mut reader);
    let line = read_next_line(&mut reader);
    let (_, [program]) = re2.captures(&line).unwrap().extract();

    let register_a = register_a.parse::<u64>().unwrap();
    let register_b = register_b.parse::<u64>().unwrap();
    let register_c = register_c.parse::<u64>().unwrap();
    let program: Vec<u8> = program
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let machine = Machine::new(register_a, register_b, register_c, program);
    machine
}
