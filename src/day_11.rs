#![allow(dead_code)]
use std::collections::HashMap;

pub fn part_a() -> u128 {
    solve(25)
}

pub fn part_b() -> u128 {
    solve(75)
}

pub fn solve(number_of_runs: u32) -> u128 {
    let mut input = include_str!("inputs/input11.txt").lines().next().unwrap().split(' ').map(|n| (n.parse().unwrap(),1)).collect::<HashMap<u128,u128>>();
    let split_table = (1..u128::MAX.ilog10()).map(|d| (10_u128.pow(d),10_u128.pow(d/2),d%2==0)).collect::<Vec<_>>();

    for _ in 0..number_of_runs {
        let mut result = HashMap::new();
        for (number, count) in input.iter() {
            if *number == 0 {
                *result.entry(1).or_insert(0) += count;
            }
            else {
                for (threshold, mul, should_split) in split_table.iter() {
                    if *number < *threshold {
                        if *should_split {
                            *result.entry(number / mul).or_insert(0) += count;
                            *result.entry(number % mul).or_insert(0) += count;
                        } else {
                            *result.entry(number * 2024).or_insert(0) += count;
                        }
                        break;
                    }
                }
            }
        }
        input = result;
    }
    input.values().sum()
}