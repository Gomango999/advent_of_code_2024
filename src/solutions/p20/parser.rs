use std::fs::File;
use std::io::{self, BufRead};

type Grid = Vec<Vec<char>>;

#[derive(Clone)]
pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub grid: Grid,
}

impl Maze {
    fn new(mut grid: Grid) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == 'S' {
                    start = (x, y);
                    grid[y][x] = '.';
                } else if grid[y][x] == 'E' {
                    end = (x, y);
                    grid[y][x] = '.';
                }
            }
        }
        assert_ne!(start, (0, 0));
        assert_ne!(end, (0, 0));

        Maze {
            height,
            width,
            start,
            end,
            grid,
        }
    }

    pub fn is_inside(&self, (x, y): (isize, isize)) -> bool {
        (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y)
    }
}

pub fn parse() -> Maze {
    let file = File::open("src/solutions/p20/in.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut grid: Grid = vec![];
    for line in reader.lines() {
        let row = line.unwrap().chars().collect();
        grid.push(row)
    }
    Maze::new(grid)
}
