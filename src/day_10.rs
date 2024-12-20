#![allow(dead_code)]
use std::collections::HashSet;
use nalgebra::{vector, Vector2};

pub fn part_a() -> usize {
    count_trail_heads_on_map(false)
}

pub fn part_b() -> usize {
    count_trail_heads_on_map(true)
}

fn count_trail_heads_on_map(count_unique_heads: bool) -> usize{
    let map = read_map();
    let map_size = Vector2::new(map[0].len() as i32, map.len() as i32);
    let mut known_heads: HashSet<Vector2<i32>> = HashSet::new();
    let mut head_sum = 0;

    for y in 0..map_size.y {
        for x in 0..map_size.x  {
            known_heads.clear();
            head_sum += count_trail_heads(0,Vector2::new(x, y), &map_size, &map, &mut known_heads, count_unique_heads);
        }
    }
    head_sum
}

fn count_trail_heads(expected_height: u8, position: Vector2<i32>, map_size: &Vector2<i32>, map: &Vec<Vec<u8>>,known_heads : &mut HashSet<Vector2<i32>>, count_unique_heads: bool) -> usize
{
    let mut head_count =0;
    if map[position.y as usize][position.x as usize] == expected_height {
        if expected_height == 9 {
            if known_heads.insert(position) || count_unique_heads { head_count += 1; }
        } else {
            let steps: [nalgebra::Vector2<i32>; 4] = [vector![-1, 0],vector![1, 0],vector![0, -1],vector![0, 1],];
            for step in steps {
                let next_step = position + step;
                if next_step.x >= 0 && next_step.x < map_size.x && next_step.y >= 0 && next_step.y < map_size.y {
                    head_count += count_trail_heads(expected_height + 1, next_step, map_size, map, known_heads, count_unique_heads);
                }
            }
        }
    }
    head_count
}

fn read_map() -> Vec<Vec<u8>> {
    let input = include_str!("inputs/input10.txt");
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}