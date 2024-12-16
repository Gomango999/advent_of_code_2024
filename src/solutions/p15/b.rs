mod parser;
mod room;

pub fn solve() {
    let (mut room, instructions) = parser::parse();

    for instruction in instructions {
        room.update(&instruction);
    }

    let gps_sum = room.compute_gps_sum();
    println!("{gps_sum}");
}
