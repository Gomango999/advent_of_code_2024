mod p01;

use aoc_2024::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1)
    });
    match config {
        Config { day: 1, part: 'a' } => p01::a::run(),
        Config { day: 1, part: 'b' } => p01::b::run(),
        _ => (),
    }
}
