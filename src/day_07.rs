#![allow(dead_code)]
pub fn part_a() -> u64{
    get_calibration_result(2)
}
pub fn part_b() -> u64{
    get_calibration_result(3)
}

pub fn get_calibration_result(number_of_operators : u32) -> u64{
    let my_str = include_str!("inputs/input07.txt");
    let mut sum = 0;

    for line in my_str.lines(){
        if let Some((test_value,inputs)) = line.split_once(':'){
            let test_value:u64 = test_value.parse().unwrap();
            let inputs  : Vec<_> = inputs
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect();

            for operator_instruction in 0..=(number_of_operators.pow((inputs.len() - 1) as u32)) {
                    if evaluate(operator_instruction, &inputs, number_of_operators) == test_value {
                        sum += test_value;
                        break;
                    }
            }
        } else {
            panic!("Row must contain 2 Elements seperated with ':'")
        }
    }

    sum
}

pub fn evaluate(mut operator_instruction: u32, inputs : &[u64], number_of_operators: u32) -> u64 {

    if !matches!(inputs.len(), 2..=32) {
        panic!("inputs must be greater or equal than 2 and smaller or equal than 32")
    }

    inputs
        .iter()
        .skip(1)
        .fold(
            inputs[0],
             |acc, input| {
                 let operator_index = operator_instruction % number_of_operators;
                 let res = match operator_index {
                     0 => acc+input,
                     1 => acc*input,
                     _ => {
                         let mut curr_input = *input;
                         let mut factor = 1;
                         while curr_input > 0 {
                             curr_input /= 10;
                             factor*=10;
                         }
                         acc * factor + input
                     }
                 };

                 operator_instruction /= number_of_operators;
                 res
             })
}

