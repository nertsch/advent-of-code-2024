use std::collections::{HashMap, HashSet};
use std::mem::take;

pub fn part_a() -> usize {
    let mut connected_computers_by_computer = HashMap::<String, HashSet<String>>::new();

    for line in include_str!("inputs/input23.txt").lines() {
        let Some((computer_1, computer_2)) = line.split_once("-") else {
            panic!("Invalid line: [{}]", line)
        };

        connected_computers_by_computer
            .entry(computer_1.to_owned())
            .or_insert_with(|| HashSet::<_>::new())
            .insert(computer_2.to_owned());

        connected_computers_by_computer
            .entry(computer_2.to_owned())
            .or_insert_with(|| HashSet::<_>::new())
            .insert(computer_1.to_owned());
    }

    let mut visited_computers = HashSet::<_>::new();

    let mut networks = Vec::<_>::new();

    for (computer, connected_computers) in connected_computers_by_computer.iter() {
        visited_computers.insert(computer.clone());
        let mut connected_computers = connected_computers.clone();
        connected_computers.retain(|c| !visited_computers.contains(c));
        if connected_computers.len() >= 2 {
            for (i,c1) in connected_computers
                .iter()
                .enumerate()
                .take(connected_computers.len() - 1)
            {
                for c2 in connected_computers.iter().skip(i+1) {
                    if let Some(connected_to_c1) = connected_computers_by_computer.get(c1) {
                        if connected_to_c1.contains(c2) {
                            networks.push((computer.to_owned(), c1.to_owned(), c2.to_owned()));
                        }
                    }
                }
            }
        }
    }

    networks.retain(|n| n.0.starts_with("t") || n.1.starts_with("t") || n.2.starts_with("t"));

    networks.len()
}
