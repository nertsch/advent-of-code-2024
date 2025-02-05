use regex::Regex;

pub fn part_a() -> i64 {
    solve(0)
}

pub fn part_b() -> i64 {
    solve(10000000000000)
}

fn solve(prize_offset: u64) -> i64 {
    let machine_configurations = read_machine_configurations(prize_offset);

    let mut total_cost: i64 = 0;

    for c in machine_configurations.iter() {
        let a0 = c.button_a.0 as f64;
        let a1 = c.button_a.1 as f64;
        let b0 = c.button_b.0 as f64;
        let b1 = c.button_b.1 as f64;
        let p0 = c.prize.0 as f64;
        let p1 = c.prize.1 as f64;
        let b = (a0 * p1 - a1 * p0) / (a0 * b1 - a1 * b0);
        let a = (p0 - b * b0) / a0;
        if a.fract() == 0f64 && b.fract() == 0f64 {
            total_cost += 3 * a as i64 + b as i64;
        }
    }
    total_cost
}

struct MachineConfiguration {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u64, u64),
}

fn read_machine_configurations(prize_offset: u64) -> Vec<MachineConfiguration> {
    let input = include_str!("inputs/input13.txt");

    let machine_regex = Regex::new(r"(?s)Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\s+Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)\s+Prize: X=(?<px>\d+), Y=(?<py>\d+)\s*").unwrap();

    let mut result = Vec::<_>::new();

    for capture in machine_regex.captures_iter(input) {
        result.push(MachineConfiguration {
            button_a: (
                capture.name("ax").unwrap().as_str().parse().unwrap(),
                capture.name("ay").unwrap().as_str().parse().unwrap(),
            ),
            button_b: (
                capture.name("bx").unwrap().as_str().parse().unwrap(),
                capture.name("by").unwrap().as_str().parse().unwrap(),
            ),
            prize: (
                capture.name("px").unwrap().as_str().parse::<u64>().unwrap() + prize_offset,
                capture.name("py").unwrap().as_str().parse::<u64>().unwrap() + prize_offset,
            ),
        });
    }

    result
}
