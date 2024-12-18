use super::parser;

pub fn solve() {
    let mut machine = parser::parse();

    machine.run(false);
    machine.print_output();
}
