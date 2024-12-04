mod p01;
mod p02;

use aoc_2024::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1)
    });
    match config {
        Config { day: 1, part: 'a' } => p01::a::run(),
        Config { day: 1, part: 'b' } => p01::b::run(),
        Config { day: 2, part: 'a' } => p02::a::run(),
        Config { day: 2, part: 'b' } => p02::b::run(),
        _ => (),
    }
}