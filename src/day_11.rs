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

    for _ in 0..number_of_runs {
        let mut result = HashMap::new();
        for (number, count) in input.iter() {
            if *number == 0 {
                *result.entry(1).or_insert(0) += count;
            }
            else {
                let number_of_digits = number.ilog10()+1;
                if number_of_digits % 2 == 0 {
                    let mul = 10u128.pow(number_of_digits/2);
                    *result.entry(number / mul).or_insert(0) += count;
                    *result.entry(number % mul).or_insert(0) += count;
                } else {
                    *result.entry(number * 2024).or_insert(0) += count;
                }
            }
        }
        input = result;
    }
    input.values().sum()
}