use std::collections::{HashMap, HashSet};
use std::mem::take;

pub fn part_a() -> usize {
    let mut connected_computers_by_computer = read_connected_computers_by_computer();

    let mut visited_computers = HashSet::<_>::new();
    let mut networks = Vec::<_>::new();

    for (computer, connected_computers) in connected_computers_by_computer.iter() {
        visited_computers.insert(computer.clone());
        let mut connected_computers = connected_computers.clone();
        connected_computers.retain(|c| !visited_computers.contains(c));
        if connected_computers.len() >= 2 {
            for (i, c1) in connected_computers
                .iter()
                .enumerate()
                .take(connected_computers.len() - 1)
            {
                for c2 in connected_computers.iter().skip(i + 1) {
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

pub fn part_b() -> String {
    let mut connected_computers_by_computer = read_connected_computers_by_computer();

    let mut networks = Vec::<HashSet<String>>::new();

    for (computer, connected_computers) in connected_computers_by_computer.iter() {
        for connected_computer in connected_computers {
            let mut no_network_found = true;
            for network in networks.iter_mut(){
                if network.contains(computer) {
                    if network.iter().all(|c| connected_computers_by_computer.get(c).unwrap().contains(connected_computer)) {
                        network.insert(connected_computer.to_owned());
                        no_network_found = false;
                    }
                }
                if network.contains(connected_computer) {
                    if network.iter().all(|c| connected_computers_by_computer.get(c).unwrap().contains(computer)) {
                        network.insert(computer.to_owned());
                        no_network_found = false;
                    }
                }
            }
            if no_network_found {
                networks.push(HashSet::from([computer.to_owned(), connected_computer.to_owned()]));
            }
        }
    }

    let biggest_network =  networks.iter().max_by(|a,b| a.len().cmp(&b.len())).unwrap();
    let mut biggest_network = biggest_network.iter().collect::<Vec<_>>();
    biggest_network.sort();
    biggest_network.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join(",")

}

fn read_connected_computers_by_computer() -> HashMap<String, HashSet<String>> {
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

    connected_computers_by_computer
}
