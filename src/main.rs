mod solutions;
use aoc_2024::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1)
    });
    match config {
        Config { day: 1, part: 'a' } => solutions::p01::a::solve(),
        Config { day: 1, part: 'b' } => solutions::p01::b::solve(),
        Config { day: 2, part: 'a' } => solutions::p02::a::solve(),
        Config { day: 2, part: 'b' } => solutions::p02::b::solve(),
        Config { day: 3, part: 'a' } => solutions::p03::a::solve(),
        Config { day: 3, part: 'b' } => solutions::p03::b::solve(),
        Config { day: 4, part: 'a' } => solutions::p04::a::solve(),
        Config { day: 4, part: 'b' } => solutions::p04::b::solve(),
        Config { day: 5, part: 'a' } => solutions::p05::a::solve(),
        Config { day: 5, part: 'b' } => solutions::p05::b::solve(),
        Config { day: 6, part: 'a' } => solutions::p06::a::solve(),
        Config { day: 6, part: 'b' } => solutions::p06::b::solve(),
        Config { day: 7, part: 'a' } => solutions::p07::a::solve(),
        Config { day: 7, part: 'b' } => solutions::p07::b::solve(),
        Config { day: 8, part: 'a' } => solutions::p08::a::solve(),
        Config { day: 8, part: 'b' } => solutions::p08::b::solve(),
        Config { day: 9, part: 'a' } => solutions::p09::a::solve(),
        Config { day: 9, part: 'b' } => solutions::p09::b::solve(),
        Config { day: 10, part: 'a' } => solutions::p10::a::solve(),
        Config { day: 10, part: 'b' } => solutions::p10::b::solve(),
        Config { day: 11, part: 'a' } => solutions::p11::a::solve(),
        Config { day: 11, part: 'b' } => solutions::p11::b::solve(),
        Config { day: 12, part: 'a' } => solutions::p12::a::solve(),
        Config { day: 12, part: 'b' } => solutions::p12::b::solve(),
        Config { day: 13, part: 'a' } => solutions::p13::a::solve(),
        Config { day: 13, part: 'b' } => solutions::p13::b::solve(),
        Config { day: 14, part: 'a' } => solutions::p14::a::solve(),
        Config { day: 14, part: 'b' } => solutions::p14::b::solve(),
        Config { day: 15, part: 'a' } => solutions::p15::a::solve(),
        Config { day: 15, part: 'b' } => solutions::p15::b::solve(),
        Config { day: 16, part: 'a' } => solutions::p16::a::solve(),
        Config { day: 16, part: 'b' } => solutions::p16::b::solve(),
        Config { day: 17, part: 'a' } => solutions::p17::a::solve(),
        Config { day: 17, part: 'b' } => solutions::p17::b::solve(),
        Config { day: 18, part: 'a' } => solutions::p18::a::solve(),
        Config { day: 18, part: 'b' } => solutions::p18::b::solve(),
        Config { day: 19, part: 'a' } => solutions::p19::a::solve(),
        Config { day: 19, part: 'b' } => solutions::p19::b::solve(),
        Config { day: 20, part: 'a' } => solutions::p20::a::solve(),
        Config { day: 20, part: 'b' } => solutions::p20::b::solve(),
        Config { day: 21, part: 'a' } => solutions::p21::a::solve(),
        Config { day: 21, part: 'b' } => solutions::p21::b::solve(),
        Config { day: 22, part: 'a' } => solutions::p22::a::solve(),
        Config { day: 22, part: 'b' } => solutions::p22::b::solve(),
        Config { day: 23, part: 'a' } => solutions::p23::a::solve(),
        Config { day: 23, part: 'b' } => solutions::p23::b::solve(),
        _ => {
            eprintln!("Not implemented yet!");
            std::process::exit(1)
        }
    }
}
