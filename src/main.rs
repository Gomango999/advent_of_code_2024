mod p01;
mod p02;
mod p03;
mod p04;
mod p05;

use aoc_2024::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1)
    });
    match config {
        Config { day: 1, part: 'a' } => p01::a::solve(),
        Config { day: 1, part: 'b' } => p01::b::solve(),
        Config { day: 2, part: 'a' } => p02::a::solve(),
        Config { day: 2, part: 'b' } => p02::b::solve(),
        Config { day: 3, part: 'a' } => p03::a::solve(),
        Config { day: 3, part: 'b' } => p03::b::solve(),
        Config { day: 4, part: 'a' } => p04::a::solve(),
        Config { day: 4, part: 'b' } => p04::b::solve(),
        Config { day: 5, part: 'a' } => p05::a::solve(),
        Config { day: 5, part: 'b' } => p05::b::solve(),
        _ => (),
    }
}
