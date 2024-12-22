use std::collections::HashMap;
use crate::day_12::add;

pub fn part_a() -> i32 {
    let numeric_keypad: HashMap<_,_> = [
        ('A', (0,0)), ('0', (1,0)), (' ', (2,0)),
        ('3', (0,1)), ('2', (1,1)), ('1', (2,1)),
        ('6', (0,2)), ('5', (1,2)), ('4', (2,2)),
        ('9', (0,3)), ('8', (1,3)), ('7', (2,3))
    ].into_iter().collect();

    let control_keypad: HashMap<_,_> = [
        ('A', (0,0)), ('^', (1,0)), (' ', (2,0)),
        ('>', (0,-1)), ('v', (1,-1)), ('<', (2,-1)),
    ].into_iter().collect();

    include_str!("inputs/input21.txt")
        .lines()
        .map(|l|calculate_code_complexity(l, &numeric_keypad, &control_keypad ) )
        .sum()
}

fn calculate_code_complexity(code: &str, numeric_keypad: &HashMap<char, (i32,i32)>, control_keypad: &HashMap<char, (i32,i32)>) -> i32 {
    let keys_to_press = code.chars().collect::<Vec<char>>();

    if keys_to_press.last().map(|c| *c) != Some('A') {
        panic!("Invalid code!")
    }

    let mut movements = calculate_keypad_movements(&numeric_keypad, &code.chars().collect::<Vec<char>>());

    for i in 1..=2 {
        print!("{}", i);
        movements = calculate_keypad_movements(&control_keypad, &movements);
        println!(" {}", movements.len());
    }

    let numeric_code_part: i32 = code[..code.len()-1].parse().unwrap();
    println!("{} * {}", movements.len(), numeric_code_part);
    numeric_code_part * movements.len() as i32
}


fn calculate_keypad_movements(position_by_key: &HashMap<char, (i32,i32)>, keys_to_press: &[char]) -> Vec<char> {
    let mut current_position = *position_by_key.get(&'A').unwrap();
    let mut keypad_movements = Vec::<_>::new();

    let gap = *position_by_key.get(&' ').unwrap();

    for key in keys_to_press {
        let new_position = position_by_key.get(key).expect("key not found in keypad");
        let dx = new_position.0 - current_position.0;
        let dy = new_position.1 - current_position.1;

        let movements_y = std::iter::repeat(if dy > 0 {'^'} else {'v'}).take(dy.abs() as usize);
        let movements_x = std::iter::repeat(if dx > 0 {'<'} else {'>'}).take(dx.abs() as usize);

        let mut prefer_x = dx > 0;

        if dx != 0 && dy != 0 && ((new_position.0, current_position.1) == gap || (current_position.0, new_position.1) == gap) {
            prefer_x = !prefer_x;
        }

        if prefer_x {
            keypad_movements.extend(movements_x);
            keypad_movements.extend(movements_y);
        }
        else {
            keypad_movements.extend(movements_y);
            keypad_movements.extend(movements_x);
        }
        keypad_movements.push('A');
        current_position = *new_position;
    }

    keypad_movements
}

pub fn part_b() -> i32 {
    0
}