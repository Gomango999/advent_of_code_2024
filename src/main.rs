mod p01;
mod p02;
mod p03;
mod p04;
mod p05;
mod p06;
mod p07;
mod p08;
mod p09;
mod p10;
mod p11;
mod p12;

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
        Config { day: 6, part: 'a' } => p06::a::solve(),
        Config { day: 6, part: 'b' } => p06::b::solve(),
        Config { day: 7, part: 'a' } => p07::a::solve(),
        Config { day: 7, part: 'b' } => p07::b::solve(),
        Config { day: 8, part: 'a' } => p08::a::solve(),
        Config { day: 8, part: 'b' } => p08::b::solve(),
        Config { day: 9, part: 'a' } => p09::a::solve(),
        Config { day: 9, part: 'b' } => p09::b::solve(),
        Config { day: 10, part: 'a' } => p10::a::solve(),
        Config { day: 10, part: 'b' } => p10::b::solve(),
        Config { day: 11, part: 'a' } => p11::a::solve(),
        Config { day: 11, part: 'b' } => p11::b::solve(),
        Config { day: 12, part: 'a' } => p12::a::solve(),
        Config { day: 12, part: 'b' } => p12::b::solve(),
        _ => {
            eprintln!("Not implemented yet!");
            std::process::exit(1)
        }
    }
}
