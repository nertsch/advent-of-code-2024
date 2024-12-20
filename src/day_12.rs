#![allow(dead_code)]
use std::collections::{HashSet};

pub fn part_a() -> i32 {
    calculate_price(false)
}

pub fn part_b() -> i32 {
    calculate_price(true)
}
pub fn calculate_price(use_edges_instead_of_perimeter: bool) -> i32 {
    let input = include_str!("inputs/input12.txt");
    let map = Map::new(input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>());

    let mut known_fields = HashSet::<(i32,i32)>::new();
    let mut total_cost= 0 ;

    for y in 0..map.height as i32 {
        for x in 0..map.width as i32 {
            let mut area = 0;
            let mut perimeter = 0;
            let mut corner_count = 0;
            traverse_area(&map, (y,x).into(), &mut area, &mut perimeter, &mut corner_count, &mut known_fields);
            // For every polygon, the number of corners is equal to the number of edges.
            let cost = area * if use_edges_instead_of_perimeter { corner_count } else { perimeter };
            total_cost += cost;
        }
    }

    total_cost
}

pub fn add(a: &(i32,i32), b: &(i32,i32)) -> (i32,i32) {
    (a.0+b.0,a.1+b.1)
}

struct Map{
    field: Vec<Vec<char>>,
    height: usize,
    width: usize
}

impl Map{
    pub fn new(field: Vec<Vec<char>>) -> Self {
        Map {
            height: field.len(),
            width: field[0].len(),
            field
        }
    }
}
impl Map {
    pub fn get_field(&self, location: &(i32,i32)) -> Option<char> {
        if location.1 >= 0 && location.1 < self.field.len() as i32 && location.0 >= 0 && location.0 < self.field[0].len() as i32 {
            Some(self.field[location.1 as usize][location.0 as usize])
        }
        else {
            None
        }
    }
}

fn traverse_area(map: &Map, current_location: (i32,i32), area: &mut i32, perimeter: &mut i32, corner_count: &mut i32, known_fields: &mut HashSet<(i32, i32)>) {
    if !known_fields.insert(current_location) {
        return;}

    let char = map.get_field(&current_location).unwrap();
    *area+=1;

    let n = map.get_field(&add(&current_location,&(0,-1))).is_some_and(|c| c == char);
    let e = map.get_field(&add(&current_location,&(1,0))).is_some_and(|c| c == char);
    let s = map.get_field(&add(&current_location,&(0,1))).is_some_and(|c| c == char);
    let w = map.get_field(&add(&current_location,&(-1,0))).is_some_and(|c| c == char);
    let ne = map.get_field(&add(&current_location,&(1,-1))).is_some_and(|c| c == char);
    let se = map.get_field(&add(&current_location,&(1,1))).is_some_and(|c| c == char);
    let sw = map.get_field(&add(&current_location,&(-1,1))).is_some_and(|c| c == char);
    let nw = map.get_field(&add(&current_location,&(-1,-1))).is_some_and(|c| c == char);

    if !n && !w { *corner_count +=1 }
    if !n && !e { *corner_count +=1 }
    if !s && !w { *corner_count +=1 }
    if !s && !e { *corner_count +=1 }
    if n && w && !nw { *corner_count +=1 }
    if n && e && !ne { *corner_count +=1 }
    if s && w && !sw { *corner_count +=1 }
    if s && e && !se { *corner_count +=1 }

    for step in [(-1,0),(1,0),(0,-1),(0,1)] {
        let new_location= add(&current_location, &step);
        if let Some(new_char) = map.get_field(&new_location) {
            if new_char == char {
                traverse_area(map, new_location, area, perimeter, corner_count, known_fields);
            }
            else {
                *perimeter+=1;
            }
        } else {
            *perimeter+=1;
        }
    }
}