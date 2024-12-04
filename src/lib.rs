pub struct Config {
    pub day: u32,
    pub part: char,
}

impl Config {
    pub fn build() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 3 {
            return Err(format!("Usage: {} <day> [a|b]", args[0]));
        }

        let day = args[1]
            .parse::<u32>()
            .map_err(|_| format!("Invalid day: {}", args[1]))?;

        if !(1..=25).contains(&day) {
            return Err(format!(
                "Error: Invalid day: {}. Must be in the range [1..25]",
                day
            ));
        }

        let part = args[2]
            .parse::<char>()
            .map_err(|_| format!("Error: Invalid part: {}", args[2]))?;

        if part != 'a' && part != 'b' {
            return Err(format!(
                "Error: Invalid problem part: {}. Must be either 'a' or 'b'",
                part
            ));
        }

        return Ok(Config { day, part });
    }
}
