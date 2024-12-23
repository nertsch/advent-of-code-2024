use nalgebra::Vector2;

pub fn part_a() -> i32 {
    let machine_configurations = read_machine_configurations();
    0
}

struct MachineConfiguration {
    distance_a: Vector2<i32>,
    distance_b: Vector2<i32>,
    price_location: Vector2<i32>,
}

fn read_machine_configurations() -> Vec<MachineConfiguration> {
    let mut input_lines = include_str!("inputs/input13.txt").lines();
    let mut machine_configurations = Vec::<_>::new();

    while {
        let distance_a = read_distance("Button A", '+', input_lines.next().unwrap());
        let distance_b = read_distance("Button B", '+', input_lines.next().unwrap());
        let price_location = read_distance("Prize", '=', input_lines.next().unwrap());

        machine_configurations.push(MachineConfiguration {
            distance_a,
            distance_b,
            price_location,
        });

        input_lines.next().is_some()
    } {}

    machine_configurations
}

fn read_distance(expected_prelude: &str, dimension_delimiter: char, line: &str) -> Vector2<i32> {
    line.split_once(':')
        .filter(|(prelude, _)| *prelude == expected_prelude)
        .map(|(_, v)| v)
        .map(str::trim)
        .map(|s| s.split_once(','))
        .flatten()
        .map(|(x, y)| {
            (
                read_dimension("X", dimension_delimiter, str::trim(x)),
                read_dimension("Y", dimension_delimiter, str::trim(y)),
            )
        })
        .map(|(x, y)| Vector2::<_>::new(x, y))
        .expect("invalid line")
}

fn read_dimension(expected_prelude: &str, delimiter: char, input: &str) -> i32 {
    input
        .split_once(delimiter)
        .filter(|(prelude, value)| *prelude == expected_prelude)
        .map(|(_, value)| value.parse().ok())
        .flatten()
        .expect(&format!("invalid dimension {}", input))
}

pub fn part_b() -> i32 {
    0
}
