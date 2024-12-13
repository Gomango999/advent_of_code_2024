use super::parser::*;

fn calculate_presses(config: &Config) -> Option<(i64, i64)> {
    let Config {
        a_button,
        b_button,
        prize,
    } = config;

    // the number of B button presses is (ax * py - ay * px) / (ax * by - ay * bx)
    let b_presses_top: i64 = a_button.x * prize.y - a_button.y * prize.x;
    let b_presses_bot: i64 = a_button.x * b_button.y - a_button.y * b_button.x;
    if b_presses_top % b_presses_bot != 0 || b_presses_top / b_presses_bot < 0 {
        return None;
    }
    let b_presses = b_presses_top / b_presses_bot;

    let a_presses_top = prize.x - b_presses * b_button.x;
    let a_presses_bot = a_button.x;
    if a_presses_top % a_presses_bot != 0 || a_presses_top / a_presses_bot < 0 {
        return None;
    }
    let a_presses = a_presses_top / a_presses_bot;

    Some((a_presses, b_presses))
}

pub fn solve() {
    let configs = parse();

    let mut total = 0;
    for mut config in configs {
        // correct for the error in part b.
        const OFFSET: i64 = 10000000000000;
        config.prize = config.prize + Vec2::new(OFFSET, OFFSET);

        if let Some((a_presses, b_presses)) = calculate_presses(&config) {
            total += 3 * a_presses + b_presses;
        }
    }
    println!("{total}")
}
