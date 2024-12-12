use super::parser;

struct Plot {
    area: u64,
    sides: u64,
}

fn is_inside((y, x): (i32, i32), garden: &Vec<Vec<char>>) -> bool {
    let n = garden.len() as i32;
    let m = garden[0].len() as i32;
    (0..n).contains(&y) && (0..m).contains(&x)
}

/// Helper function to add two tuple pairs together
fn add((y1, x1): (i32, i32), (y2, x2): (i32, i32)) -> (i32, i32) {
    (y1 + y2, x1 + x2)
}

/// Returns how many outer corners a plant produces
/// E.g. The A in the middle produces 2 outer corners.
/// ...
/// .A.
/// .A.
fn num_outer_corner(y: i32, x: i32, garden: &Vec<Vec<char>>) -> u64 {
    let adj = [
        ((1, 0), (0, 1)),
        ((0, 1), (-1, 0)),
        ((-1, 0), (0, -1)),
        ((0, -1), (1, 0)),
    ];
    let mut total = 0;
    let curr = (y, x);
    for (off1, off2) in adj {
        let next1 = add(curr, off1);
        let next2 = add(curr, off2);

        let is_fence1 = !is_inside(next1, garden)
            || garden[next1.0 as usize][next1.1 as usize]
                != garden[curr.0 as usize][curr.1 as usize];
        let is_fence2 = !is_inside(next2, garden)
            || garden[next2.0 as usize][next2.1 as usize]
                != garden[curr.0 as usize][curr.1 as usize];

        if is_fence1 && is_fence2 {
            total += 1;
        }
    }
    total
}

/// Returns how many inner corners a plant produces
/// E.g. The A in the middle produces 2 inner corners
/// .A.
/// .AA
/// .A.
fn num_inner_corner(y: i32, x: i32, garden: &Vec<Vec<char>>) -> u64 {
    let adj = [
        ((1, 0), (0, 1)),
        ((0, 1), (-1, 0)),
        ((-1, 0), (0, -1)),
        ((0, -1), (1, 0)),
    ];

    let mut total = 0;
    let curr = (y, x);
    for (off1, off2) in adj {
        let next1 = add(curr, off1);
        let next2 = add(curr, add(off1, off2)); // the diagonal between the two offsets.
        let next3 = add(curr, off2);

        let is_diff1 = !is_inside(next1, garden)
            || garden[next1.0 as usize][next1.1 as usize]
                != garden[curr.0 as usize][curr.1 as usize];
        let is_diff2 = !is_inside(next2, garden)
            || garden[next2.0 as usize][next2.1 as usize]
                != garden[curr.0 as usize][curr.1 as usize];
        let is_diff3 = !is_inside(next3, garden)
            || garden[next3.0 as usize][next3.1 as usize]
                != garden[curr.0 as usize][curr.1 as usize];

        if !is_diff1 && is_diff2 && !is_diff3 {
            total += 1;
        }
    }
    total
}

/// For a given (x,y), dfs will return the area and perimeter of that plot.
fn find_plot(y: usize, x: usize, garden: &Vec<Vec<char>>, seen: &mut Vec<Vec<bool>>) -> Plot {
    if seen[y][x] {
        return Plot { area: 0, sides: 0 };
    }
    seen[y][x] = true;

    let num_outer_corners = num_outer_corner(y as i32, x as i32, garden);
    let num_inner_corners = num_inner_corner(y as i32, x as i32, garden);
    let sides = num_outer_corners + num_inner_corners;
    let mut plot = Plot { area: 1, sides };

    let adj = [(1, 0), (0, 1), (0, -1), (-1, 0)];
    for offset in adj {
        let ny = y as i32 + offset.0;
        let nx = x as i32 + offset.1;
        if !is_inside((ny, nx), garden) {
            continue;
        }

        let ny = ny as usize;
        let nx = nx as usize;
        if garden[ny][nx] == garden[y][x] {
            let adj_plot = find_plot(ny, nx, garden, seen);
            plot.area += adj_plot.area;
            plot.sides += adj_plot.sides;
        }
    }
    plot
}

pub fn solve() {
    let garden = parser::parse();

    let n = garden.len();
    let m = garden[0].len();

    let mut seen = vec![vec![false; m]; n];

    let mut total: u64 = 0;
    for y in 0..n {
        for x in 0..m {
            let plot = find_plot(y, x, &garden, &mut seen);
            total += plot.area * plot.sides;
        }
    }
    println!("{total}")
}
