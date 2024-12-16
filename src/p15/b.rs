use super::parser_b;

pub fn solve() {
    let (mut room, instructions) = parser_b::parse();

    for instruction in instructions {
        room.update(&instruction);
    }

    // print!("{room}");

    let gps_sum = room.compute_gps_sum();
    println!("{gps_sum}");
}
