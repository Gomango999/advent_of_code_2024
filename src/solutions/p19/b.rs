use super::parser::*;

fn num_designs_possible(design: &Pattern, towels: &Vec<Pattern>) -> u64 {
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
    dp[design.len()]
}

pub fn solve() {
    let Input { towels, designs } = parse();

    let num_possible: u64 = designs
        .into_iter()
        .map(|design| num_designs_possible(&design, &towels))
        .sum();
    println!("{num_possible}")
}
