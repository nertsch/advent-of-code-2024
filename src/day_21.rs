use std::collections::HashMap;

pub fn part_a() -> u64 {
    calculate_total_complexity(2)
}

pub fn part_b() -> u64 {
    calculate_total_complexity(25)
}

pub fn calculate_total_complexity(number_of_control_pads: u8) -> u64 {
    let numeric_keypad: HashMap<_, _> = [
        ('A', (0, 0)),
        ('0', (1, 0)),
        (' ', (2, 0)),
        ('3', (0, 1)),
        ('2', (1, 1)),
        ('1', (2, 1)),
        ('6', (0, 2)),
        ('5', (1, 2)),
        ('4', (2, 2)),
        ('9', (0, 3)),
        ('8', (1, 3)),
        ('7', (2, 3)),
    ]
    .into_iter()
    .collect();

    let control_keypad: HashMap<_, _> = [
        ('A', (0, 0)),
        ('^', (1, 0)),
        (' ', (2, 0)),
        ('>', (0, -1)),
        ('v', (1, -1)),
        ('<', (2, -1)),
    ]
    .into_iter()
    .collect();

    let mut keypads = vec![&numeric_keypad];
    for _ in 0..number_of_control_pads {
        keypads.push(&control_keypad);
    }

    let mut total_complexity = 0;
    let mut number_of_arm_movements_cache = HashMap::<_, _>::new();

    for line in include_str!("inputs/input21.txt").lines() {
        let number_of_arm_movements =
            get_number_of_arm_movements(&mut number_of_arm_movements_cache, line.chars(), &keypads);
        let numeric_code_part: u64 = line[..line.len() - 1].parse().unwrap();
        total_complexity += number_of_arm_movements * numeric_code_part;
    }

    total_complexity
}

#[derive(Eq, PartialEq, Hash)]
struct NumberOfArmMovementsCacheKey {
    pub current_key: char,
    pub next_key: char,
    pub level_id: usize,
}

fn get_number_of_arm_movements(
    number_of_arm_movements_cache: &mut HashMap<NumberOfArmMovementsCacheKey, u64>,
    keys: impl Iterator<Item = char>,
    keypads: &[&HashMap<char, (i32, i32)>],
) -> u64 {
    if keypads.len() == 0 {
        return keys.count() as u64;
    };

    let keypad = &keypads[0];
    let mut current_key = 'A';
    let mut number_of_arm_movements = 0;

    for next_key in keys {
        number_of_arm_movements += get_number_of_arm_movements_for_single_key(
            number_of_arm_movements_cache,
            current_key,
            next_key,
            keypads,
        );
        current_key = next_key;
    }

    number_of_arm_movements
}

fn get_number_of_arm_movements_for_single_key(
    number_of_arm_movements_cache: &mut HashMap<NumberOfArmMovementsCacheKey, u64>,
    current_key: char,
    next_key: char,
    keypads: &[&HashMap<char, (i32, i32)>],
) -> u64 {
    let level_id = keypads.len();
    let number_of_arm_movements_cache_key = NumberOfArmMovementsCacheKey {
        current_key,
        next_key,
        level_id,
    };
    if let Some(number_of_arm_movements) =
        number_of_arm_movements_cache.get(&number_of_arm_movements_cache_key)
    {
        return *number_of_arm_movements;
    }

    let keypad = &keypads[0];

    let gap_position = *keypad.get(&' ').unwrap();

    let current_position = keypad.get(&current_key).expect("key not found in keypad");
    let next_position = keypad.get(&next_key).expect("key not found in keypad");
    let dx = next_position.0 - current_position.0;
    let dy = next_position.1 - current_position.1;

    let movements_y = std::iter::repeat(if dy > 0 { '^' } else { 'v' }).take(dy.abs() as usize);
    let movements_x = std::iter::repeat(if dx > 0 { '<' } else { '>' }).take(dx.abs() as usize);

    let mut prefer_x = dx > 0;

    if dx != 0
        && dy != 0
        && ((next_position.0, current_position.1) == gap_position
            || (current_position.0, next_position.1) == gap_position)
    {
        prefer_x = !prefer_x;
    }

    let movements = if prefer_x {
        movements_x.chain(movements_y)
    } else {
        movements_y.chain(movements_x)
    };
    let movements = movements.chain(std::iter::once('A'));

    let number_of_arm_movements = get_number_of_arm_movements(
        number_of_arm_movements_cache,
        movements.into_iter(),
        &keypads[1..],
    );
    number_of_arm_movements_cache
        .insert(number_of_arm_movements_cache_key, number_of_arm_movements);
    number_of_arm_movements
}
