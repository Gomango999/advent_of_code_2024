use super::parser::*;

fn is_design_possible(design: &Pattern, towels: &Vec<Pattern>) -> bool {
    // Run dymamic programming. dp[x] counts the number of ways to create a
    // design all the way up to the `x`th stripe using towels
    let mut dp: Vec<u64> = vec![0; design.len() + 1];
    dp[0] = 1;
    for i in 0..design.len() {
        for towel in towels {
            if design[i..].starts_with(&towel[..]) {
                dp[i + towel.len()] += dp[i];
            }
        }
    }
    let possible = dp[design.len()] > 0;
    possible
}

pub fn solve() {
    let Input { towels, designs } = parse();

    let num_possible = designs
        .into_iter()
        .filter(|design| is_design_possible(&design, &towels))
        .count();
    println!("{num_possible}")
}
