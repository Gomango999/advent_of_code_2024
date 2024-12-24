use super::parser;

#[inline]
fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

#[inline]
fn prune(secret: u64) -> u64 {
    secret & 0xFFFFFF
}

fn evolve(mut secret: u64) -> u64 {
    secret = prune(mix(secret, secret << 6));
    secret = prune(mix(secret, secret >> 5));
    secret = prune(mix(secret, secret << 11));
    secret
}

pub fn solve() {
    let secrets = parser::parse();

    const NUM_EVOLUTIONS: usize = 2000;

    let total: u64 = secrets
        .into_iter()
        .map(|mut secret| {
            for _ in 0..NUM_EVOLUTIONS {
                secret = evolve(secret);
            }
            secret
        })
        .sum();
    println!("{}", total);
}
