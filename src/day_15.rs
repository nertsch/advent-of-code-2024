use std::collections::HashMap;

pub fn part_a() -> i32 {
    let Input { mut object_by_position, moves, mut robot_position} = read_input();

    //write_warehouse(&object_by_position, &robot_position);

    for mv in moves.iter() {
        let move_fn = get_move_fn(mv);
        let new_robot_position = move_fn(robot_position);
        if move_box_if_possible(new_robot_position, move_fn, &mut object_by_position) {
            robot_position = new_robot_position;
        }
        //write_warehouse(&object_by_position, &robot_position);
    }

    object_by_position.iter().filter_map(|e| if *e.1 == 'O' { Some(e.0.0+e.0.1*100)} else {  None }).sum()
}

fn write_warehouse(object_by_position: &HashMap<(i32, i32), char>, robot_position: &(i32,i32)){

    let mut as_string = String::new();

    for y in 0..50{
        for x in 0..50 {
            if x == robot_position.0 && y == robot_position.1 {
                as_string.push('@');
            } else {
                as_string.push(*object_by_position.get(&(x,y)).unwrap_or(&' '));
            }
        }
        as_string.push('\n');
    }

    println!("{}", as_string);
}

fn move_box_if_possible(pos: (i32, i32), move_fn: impl Fn((i32, i32)) -> (i32, i32), warehouse: &mut HashMap<(i32, i32), char>) -> bool {
    match warehouse.get(&pos) {
        None => true,
        Some('#') => false,
        Some('O') => {
            let new_position = move_fn(pos);
            if move_box_if_possible(new_position, move_fn, warehouse) {
                warehouse.remove(&pos);
                warehouse.insert(new_position, 'O');
                true
            } else {
                false
            }
        }
        _ => panic!("Invalid object!")
    }
}

fn get_move_fn(c: &char) -> impl Fn((i32, i32)) -> (i32, i32) {
    match c {
        '<' => |p: (i32, i32)| (p.0 - 1, p.1),
        '>' => |p: (i32, i32)| (p.0 + 1, p.1),
        '^' => |p: (i32, i32)| (p.0, p.1 - 1),
        'v' => |p: (i32, i32)| (p.0, p.1 + 1),
        _ => panic!()
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