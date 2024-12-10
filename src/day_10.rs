use std::collections::HashSet;
use nalgebra::{vector, Vector2};

pub fn part_a() -> usize {
    count_trail_heads(false)
}

pub fn part_b() -> usize {
    count_trail_heads(true)
}

fn count_trail_heads(count_unique_heads: bool) -> usize{
    let map = read_map();
    let map_size = Vector2::new(map[0].len() as i32, map.len() as i32);
    let mut head_collector: Vec<Vector2<i32>> = Vec::new();
    let mut head_sum = 0;

    for y in 0..map_size.y {
        for x in 0..map_size.x  {
            head_collector.clear();
            walk_trail(0,Vector2::new(x, y), &map_size, &map, &mut head_collector);
            head_sum += if count_unique_heads {
                head_collector.iter().count()
            } else {
                head_collector.iter().collect::<HashSet<_>>().iter().count()
            };
        }
    }
    head_sum
}

fn walk_trail(expected_height: u8, position: Vector2<i32>, map_size: &Vector2<i32>, map: &Vec<Vec<u8>>,head_collector : &mut Vec<Vector2<i32>>)
{
    if map[position.y as usize][position.x as usize] == expected_height {
        if expected_height == 9 {
            head_collector.push(position);
        } else {
            let steps: [nalgebra::Vector2<i32>; 4] = [vector![-1, 0],vector![1, 0],vector![0, -1],vector![0, 1],];
            for step in steps {
                let next_step = position + step;
                if next_step.x >= 0 && next_step.x < map_size.x && next_step.y >= 0 && next_step.y < map_size.y {
                    walk_trail(expected_height + 1, next_step, map_size, map, head_collector);
                }
            }
        }
    }
}

fn read_map() -> Vec<Vec<u8>> {
    let input = include_str!("inputs/input10.txt");
    input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect()
}