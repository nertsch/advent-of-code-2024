use regex::Regex;
use std::collections::HashSet;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

pub fn part_a() -> i64 {
    let mut robots = read_robots();

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots.iter_mut() {
        let px = (robot.px + 100 * robot.vx).rem_euclid(WIDTH);
        let py = (robot.py + 100 * robot.vy).rem_euclid(HEIGHT);

        if px < WIDTH / 2 && py < HEIGHT / 2 {
            q1 += 1;
        } else if px > WIDTH / 2 && py < HEIGHT / 2 {
            q2 += 1;
        } else if px > WIDTH / 2 && py > HEIGHT / 2 {
            q3 += 1;
        } else if px < WIDTH / 2 && py > HEIGHT / 2 {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

pub fn part_b() -> i64 {
    let mut robots = read_robots();
    let mut seconds_elapsed = 0;

    loop {
        if robots.iter().map(|r| (r.px, r.py)).collect::<HashSet<_>>().len() == robots.len() {
            return  seconds_elapsed;
        }
        for robot in robots.iter_mut(){
            robot.px = (robot.px + robot.vx).rem_euclid(WIDTH);
            robot.py = (robot.py + robot.vy).rem_euclid(HEIGHT);
        }
        seconds_elapsed += 1;
    }
}

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

fn read_robots() -> Vec<Robot> {
    let input = include_str!("inputs/input14.txt");

    let robot_regex = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();

    let mut result = Vec::<_>::new();

    for capture in robot_regex.captures_iter(input) {
        result.push(Robot {
            px: capture.name("px").unwrap().as_str().parse().unwrap(),
            py: capture.name("py").unwrap().as_str().parse().unwrap(),
            vx: capture.name("vx").unwrap().as_str().parse().unwrap(),
            vy: capture.name("vy").unwrap().as_str().parse().unwrap(),
        });
    }

    result
}
