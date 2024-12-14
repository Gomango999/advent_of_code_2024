use super::parser::*;

const ROOM_SIZE: Vec2 = Vec2 { x: 101, y: 103 };

/// Counts which robots go in each quadrants. Quadrants are labelled like so
/// 1|2
/// -+-
/// 3|4

fn count_quadrants(robots: &Vec<Vec2>) -> [i64; 4] {
    let mut quadrants = [0; 4];
    let cutoff = ROOM_SIZE / 2;
    for robot in robots {
        if robot.x < cutoff.x && robot.y < cutoff.y {
            quadrants[0] += 1;
        } else if robot.x < cutoff.x && robot.y > cutoff.y {
            quadrants[1] += 1;
        } else if robot.x > cutoff.x && robot.y < cutoff.y {
            quadrants[2] += 1;
        } else if robot.x > cutoff.x && robot.y > cutoff.y {
            quadrants[3] += 1;
        }
    }
    quadrants
}

pub fn solve() {
    let robots = parse();

    let robots = robots
        .into_iter()
        .map(|robot| {
            let new_pos = (robot.pos + robot.vel * 100) % ROOM_SIZE;
            new_pos
        })
        .collect();

    let quadrants = count_quadrants(&robots);
    let safety_factor: i64 = quadrants.iter().product();
    println!("{safety_factor}")
}
