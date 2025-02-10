use std::collections::HashMap;

pub fn part_a() -> i32 {
    let Input {
        mut object_by_position,
        moves,
        mut robot_position,
    } = read_input();

    for mv in moves.iter() {
        let move_fn = get_move_fn(mv);
        let new_robot_position = move_fn(robot_position);
        if clear_location(new_robot_position, move_fn, &mut object_by_position) {
            robot_position = new_robot_position;
        }
    }

    object_by_position
        .iter()
        .filter_map(|e| {
            if *e.1 == 'O' {
                Some(e.0 .0 + e.0 .1 * 100)
            } else {
                None
            }
        })
        .sum()
}

fn clear_location(
    location: (i32, i32),
    get_new_box_location: impl Fn((i32, i32)) -> (i32, i32),
    object_by_position: &mut HashMap<(i32, i32), char>,
) -> bool {
    match object_by_position.get(&location) {
        None => true,
        Some('#') => false,
        Some('O') => {
            let new_position = get_new_box_location(location);
            if clear_location(new_position, get_new_box_location, object_by_position) {
                object_by_position.remove(&location);
                object_by_position.insert(new_position, 'O');
                true
            } else {
                false
            }
        }
        _ => panic!("Invalid object!"),
    }
}

fn get_move_fn(c: &char) -> impl Fn((i32, i32)) -> (i32, i32) {
    match c {
        '<' => |p: (i32, i32)| (p.0 - 1, p.1),
        '>' => |p: (i32, i32)| (p.0 + 1, p.1),
        '^' => |p: (i32, i32)| (p.0, p.1 - 1),
        'v' => |p: (i32, i32)| (p.0, p.1 + 1),
        _ => panic!(),
    }
}

struct Input {
    object_by_position: HashMap<(i32, i32), char>,
    moves: Vec<char>,
    robot_position: (i32, i32),
}

fn read_input() -> Input {
    let mut result = Input {
        object_by_position: HashMap::<_, _>::new(),
        moves: Vec::<_>::new(),
        robot_position: (0, 0),
    };

    let input = include_str!("inputs/input15.txt");
    let mut is_reading_warehouse = true;

    for (y, line) in input.lines().enumerate() {
        if is_reading_warehouse {
            if line.len() > 0 {
                for (x, char) in line.chars().enumerate() {
                    if char != '.' {
                        if char == '@' {
                            result.robot_position = (x as i32, y as i32);
                        } else {
                            result.object_by_position.insert((x as i32, y as i32), char);
                        }
                    }
                }
            } else {
                is_reading_warehouse = false;
            }
        } else {
            result.moves.extend(line.chars());
        }
    }

    result
}
