pub fn part_a() -> u64{
    let my_str = include_str!("inputs/input07.txt");
    let mut sum = 0;

    for line in my_str.lines(){
        if let Some((test_value,inputs)) = line.split_once(':'){
            let test_value:u64 = test_value.parse().unwrap();
            let inputs  : Vec<_> = inputs
                .split_whitespace()
                .map(|i| i.parse::<u64>().unwrap())
                .collect();

            for operator_instruction in 0..=(1 << inputs.len()-1) {
                    if evaluate(operator_instruction, &inputs) == test_value {
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

pub fn evaluate(mut operator_instruction: u32, inputs : &[u64]) -> u64 {

    if !matches!(inputs.len(), 2..=32) {
        panic!("inputs must be greater or equal than 2 and smaller or equal than 32")
    }

    inputs
        .iter()
        .skip(1)
        .fold(
            inputs[0],
             |acc, input| {
                 let res = if operator_instruction & 1 == 0 { acc+input } else { acc*input};
                 operator_instruction >>= 1;
                 res
             })
}

