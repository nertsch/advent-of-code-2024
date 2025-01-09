use regex::Regex;
use std::collections::HashMap;

struct Wire {
    pub evaluate: Box<dyn Fn(&HashMap<String, Wire>) -> bool>,
}

pub fn part_a() -> u128 {
    let mut wire_value_by_wire_name = HashMap::<String, Wire>::new();

    let mut is_reading_values = true;

    for line in include_str!("inputs/input24.txt").lines() {
        if is_reading_values {
            if line.is_empty() {
                is_reading_values = false;
                continue;
            }

            let (wire_name, wire_value) = line.split_once(':').unwrap();
            let wire_value = wire_value.trim();
            let wire_value = wire_value == "1";

            wire_value_by_wire_name.insert(
                wire_name.to_owned(),
                Wire {
                    evaluate: Box::new(move |w| wire_value),
                },
            );
        } else {
            let mut split = line.split_whitespace();
            let wire_1_name = split.next().unwrap();
            let operator = split.next().unwrap();
            let wire_2_name = split.next().unwrap();
            split.next().unwrap();
            let wire_out = split.next().unwrap().to_owned();

            match operator {
                "OR" => wire_value_by_wire_name.insert(
                    wire_out,
                    Wire {
                        evaluate: Box::new(|w| {
                            (w.get(wire_1_name).unwrap().evaluate)(w)
                                || (w.get(wire_2_name).unwrap().evaluate)(w)
                        }),
                    },
                ),
                "AND" => wire_value_by_wire_name.insert(
                    wire_out,
                    Wire {
                        evaluate: Box::new(|w| {
                            (w.get(wire_1_name).unwrap().evaluate)(w)
                                && (w.get(wire_2_name).unwrap().evaluate)(w)
                        }),
                    },
                ),
                "XOR" => wire_value_by_wire_name.insert(
                    wire_out,
                    Wire {
                        evaluate: Box::new(|w| {
                            (w.get(wire_1_name).unwrap().evaluate)(w)
                                != (w.get(wire_2_name).unwrap().evaluate)(w)
                        }),
                    },
                ),
                _ => panic!(),
            };
        }
    }

    let mut wire_names = wire_value_by_wire_name
        .keys()
        .filter(|w| w.starts_with("z"))
        .collect::<Vec<_>>();

    wire_names.sort();
    wire_names.reverse();

    let mut result = 0u128;

    for wire_name in wire_names {
        result <<= 1;
        result |= if (wire_value_by_wire_name.get(wire_name).unwrap().evaluate)(&wire_value_by_wire_name) {1u128} else {0u128};
    }

    result
}
