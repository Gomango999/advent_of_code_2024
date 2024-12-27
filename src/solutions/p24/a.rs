use super::parser;

pub fn solve() {
    let mut machine = parser::parse();

    let output = machine.run();

    println!("{}", output);
}
