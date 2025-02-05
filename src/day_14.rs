use regex::Regex;
use std::collections::HashMap;

pub fn part_a() -> i64 {
    let mut robots = read_robots();
    let width = 101;
    let height = 103;

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots.iter_mut() {
        let px = (robot.px + 100 * robot.vx).rem_euclid(width);
        let py = (robot.py + 100 * robot.vy).rem_euclid(height);

        if px < width / 2 && py < height / 2 {
            q1 += 1;
        } else if px > width / 2 && py < height / 2 {
            q2 += 1;
        } else if px > width / 2 && py > height / 2 {
            q3 += 1;
        } else if px < width / 2 && py > height / 2 {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
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
