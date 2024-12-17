use std::fs::File;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

#[derive(Clone)]
pub struct Input {
    pub height: usize,
    pub width: usize,
    pub start_x: usize,
    pub start_y: usize,
    pub end_x: usize,
    pub end_y: usize,
    pub grid: Grid,
}

impl Input {
    fn new(grid: Grid) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut start_x = 0;
        let mut start_y = 0;
        let mut end_x = 0;
        let mut end_y = 0;
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == 'S' {
                    start_x = x;
                    start_y = y;
                } else if grid[y][x] == 'E' {
                    end_x = x;
                    end_y = y;
                }
            }
        }
        assert_ne!(start_x, 0);
        assert_ne!(start_y, 0);
        assert_ne!(end_x, 0);
        assert_ne!(end_x, 0);

        Input {
            height,
            width,
            start_x,
            start_y,
            end_x,
            end_y,
            grid,
        }
    }
}

pub fn parse() -> Input {
    let file = File::open("src/solutions/p16/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid: Grid = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().collect();
        grid.push(row)
    }
    Input::new(grid)
}
