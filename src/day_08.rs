use nalgebra::Vector2;
use std::collections::{HashMap, HashSet};

pub fn part_a() -> usize {
    let map = Map::read_from_file();

    let mut antinode_positions: HashSet<Vector2<i32>> = HashSet::new();

    map.process_antenna_tuples(|a1, a2| {
        let antenna_distance = a2 - a1;
        for antinode_position in [a1 - antenna_distance, a2 + antenna_distance] {
            if map.is_in_bounds(&antinode_position) {
                antinode_positions.insert(antinode_position);
            }
        }
    });

    antinode_positions.len()
}

pub fn part_b() -> usize {
    let map = Map::read_from_file();

    let mut antinode_positions: HashSet<Vector2<i32>> = HashSet::new();

    map.process_antenna_tuples(|a1, a2| {
        let antenna_distance = a2 - a1;
        let paths: [(&Vector2<i32>, Vector2<i32>); 2] =
            [(a1, -antenna_distance), (a2, antenna_distance)];
        for (antinode_position, step) in paths.into_iter() {
            let mut antinode_position: Vector2<i32> = *antinode_position;
            while map.is_in_bounds(&antinode_position) {
                antinode_positions.insert(antinode_position);
                antinode_position = antinode_position + step;
            }
        }
    });

    antinode_positions.len()
}

struct Map {
    pub antenna_positions_by_antenna_type: HashMap<char, Vec<Vector2<i32>>>,
    height: i32,
    width: i32,
}
impl Map {
    pub fn read_from_file() -> Map {
        let raw_map = include_str!("inputs/input08.txt");

        let mut antenna_positions_by_antenna_type: HashMap<char, Vec<Vector2<i32>>> =
            HashMap::new();

        let mut height = 0usize;
        let mut width = 0usize;

        for (y, line) in raw_map.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antenna_positions_by_antenna_type
                        .entry(ch)
                        .or_insert(Vec::new())
                        .push(Vector2::new(x as i32, y as i32));
                }
                width = std::cmp::max(width, x);
                height = std::cmp::max(height, y);
            }
        }

        let height = (height + 1) as i32;
        let width = (width + 1) as i32;

        Map {
            antenna_positions_by_antenna_type,
            height,
            width,
        }
    }
    pub fn is_in_bounds(&self, position: &Vector2<i32>) -> bool {
        position.x >= 0 && position.x < self.width && position.y >= 0 && position.y < self.height
    }

    pub fn process_antenna_tuples(
        &self,
        mut process_tuple: impl FnMut(&Vector2<i32>, &Vector2<i32>),
    ) {
        for antenna_positions in self.antenna_positions_by_antenna_type.values() {
            if antenna_positions.len() <= 1 {
                continue;
            }
            for a1 in antenna_positions.iter().take(antenna_positions.len() - 1) {
                for a2 in antenna_positions.iter().skip(1) {
                    if a1 == a2 {
                        continue;
                    }
                    process_tuple(a1, a2);
                }
            }
        }
    }
}
