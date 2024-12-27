use std::fs;

#[derive(Debug)]
pub struct Key {
    pub heights: [u64; 5],
}
impl Key {
    fn new(heights: [u64; 5]) -> Self {
        Self { heights: heights }
    }
}

#[derive(Debug)]
pub struct Lock {
    pub heights: [u64; 5],
}
impl Lock {
    fn new(heights: [u64; 5]) -> Self {
        Self { heights: heights }
    }
}

pub fn parse() -> (Vec<Key>, Vec<Lock>) {
    let file_path = "src/solutions/p25/in.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let (keys, locks) =
        content
            .split("\n\n")
            .fold((Vec::new(), Vec::new()), |(mut keys, mut locks), shape| {
                let is_key = shape.chars().next().unwrap() == '.';

                let lines: Vec<&str> = shape.split("\n").collect();

                let mut heights = [0, 0, 0, 0, 0];

                for (i, line) in lines.into_iter().enumerate() {
                    for (j, c) in line.chars().enumerate() {
                        if is_key {
                            if c == '.' {
                                heights[j] = 5 - i as u64;
                            };
                        } else {
                            if c == '#' {
                                heights[j] = i as u64;
                            }
                        }
                    }
                }

                if is_key {
                    keys.push(Key::new(heights));
                } else {
                    locks.push(Lock::new(heights));
                }
                (keys, locks)
            });

    (keys, locks)
}
