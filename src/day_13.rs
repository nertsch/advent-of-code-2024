use regex::Regex;

pub fn part_a() -> i32 {
    let machine_configurations = read_machine_configurations();

    for c in machine_configurations.iter() {
        let num = c.prize.1*c.button_a.0 - c.prize.0*c.button_a.1;
        let den = c.button_b.1*c.button_a.0 - c.button_b.0*c.button_a.1;
        if num % den == 0 {
            let b = num / den;

        }
    }

    0
}

struct MachineConfiguration {
    button_a: (u32, u32),
    button_b: (u32, u32),
    prize: (u32, u32),
}

fn read_machine_configurations() -> Vec<MachineConfiguration> {
    let input = include_str!("inputs/input13.txt");

    let machine_regex = Regex::new(r"(?s)Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)\s+Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)\s+Prize: X=(?<px>\d+), Y=(?<py>\d+)\s*").unwrap();

    let mut result = Vec::<_>::new();

    for capture in machine_regex.captures_iter(input) {
        println!("{:?}", capture.get(0).unwrap());
        result.push(
            MachineConfiguration{
                button_a: (capture.name("ax").unwrap().as_str().parse().unwrap(),capture.name("ay").unwrap().as_str().parse().unwrap()),
                button_b: (capture.name("bx").unwrap().as_str().parse().unwrap(),capture.name("by").unwrap().as_str().parse().unwrap()),
                prize: (capture.name("px").unwrap().as_str().parse().unwrap(),capture.name("py").unwrap().as_str().parse().unwrap()),
            }
        );
    }

    result
}

pub fn part_b() -> i32 {
    0
}