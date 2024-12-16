use super::parser_a;

pub fn solve() {
    let (mut room, instructions) = parser_a::parse();

    for instruction in instructions {
        room.update(&instruction);
    }

    let gps_sum = room.compute_gps_sum();
    println!("{gps_sum}");
}
